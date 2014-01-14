/*
 * (c) 2013 Duncan Mac-Vicar P.
 *
 * Released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package pistacho.loader;

import java.io.ByteArrayInputStream;
import java.io.DataInputStream;
import java.io.EOFException;
import java.io.InputStream;
import java.nio.charset.Charset;
import java.util.Arrays;
import pistacho.LoaderBase;
import pistacho.Module;
import pistacho.Pattern;
import pistacho.Sample;

/**
 * Format documentation:
 * http://schismtracker.org/wiki/FMODDOC.TXT
 */
public class MODLoader extends LoaderBase {

    private static final String UNEXPECTED_EOF = "Unexpected end of the file";

    @Override
    public Module loadStream(InputStream data) throws Exception {

        Module mod = new Module();

        byte[] headerArr = new byte[1080];
        data.read(headerArr, 0, 1080);

        final byte[] sig4 = "M.K.".getBytes("UTF-8");
        final byte[] sig6 = "6CHN".getBytes("UTF-8");
        final byte[] sig8 = "8CHN".getBytes("UTF-8");

        final byte[] sig = new byte[4];
        data.read(sig, 0, 4);

        if (Arrays.equals(sig, sig4)) {
            mod.setNumberOfChannels(4);
        } else if (Arrays.equals(sig, sig6)) {
            mod.setNumberOfChannels(6);
        } else if (Arrays.equals(sig, sig8)) {
            mod.setNumberOfChannels(8);
        } else {
            throw new Exception("Not a MOD file");
        }

        // read header
        InputStream header = new DataInputStream(new ByteArrayInputStream(headerArr));
        mod.setName(readNullTerminatedString(header, 20, Charset.defaultCharset()));
        for (int i = 0; i < 31; i++) {
            Sample sample = new Sample();
            sample.setName(readNullTerminatedString(header, 22, Charset.defaultCharset()));
            sample.setLength(readWord(header));
            sample.setFineTune(header.read());
            sample.setVolume(header.read());
            sample.setLoopStart(readWord(header));
            sample.setLoopEnd(readWord(header));
            mod.getSamples().add(sample);
            System.out.println(sample.getName());
        }

        // now read the pattern order / play sequences
        int numOrders = header.read();
        int unused = header.read();

        int[] orders = new int[128];

        int maxPattern = 0;
        for (int i = 0; i < 128; i++) {
            orders[i] = header.read();
            if (orders[i] < 0) {
                throw new EOFException(UNEXPECTED_EOF);
            }
            if (orders[i] > maxPattern) {
                maxPattern = orders[i];
            }
        }
        // maxPattern is an index
        int numPatterns = maxPattern + 1;
        if (numPatterns < 0) {
            throw new Exception("Invalid pattern length");
        }
        // now continue with data (header already done)
        for (int patIdx = 0; patIdx < numPatterns; patIdx++) {
            Pattern pat = new Pattern(mod.getNumberOfChannels(), 64);
            for (int rowIdx = 0; rowIdx < 64; rowIdx++) {
                for (int chanIdx = 0; chanIdx < mod.getNumberOfChannels(); chanIdx++) {
                    byte[] entry = new byte[4];
                    int ret = data.read(entry, 0, 4);
                    if (ret < 0) {
                        throw new EOFException(UNEXPECTED_EOF);
                    }
                    int sampleNumber = ((entry[0] & 0xF0) + ((entry[2] >> 4) & 0xFF)) & 0xFF;

                    int periodFrequency = ((entry[0] & 0x0F) << 8) + (entry[1] & 0xFF);
                    int effectNumber = (entry[2] & 0x0F);
                    int effectParam = entry[3] & 0xFF;

                    Sample sample = null;
                    if (sampleNumber <= 31 && sampleNumber > 0) {
                        sample = mod.getSamples().get(sampleNumber - 1);
                    }
                    pat.setSample(rowIdx, chanIdx, sample);
                    pat.setPeriodFrequency(rowIdx, chanIdx, periodFrequency);
                    // TODO add effects...

                }
            }
            mod.getPatterns().add(pat);
        }

        // add the created patterns to the order
        // (we only had the indexes until now)
        for (int i = 0; i < numOrders; i++) {
            Pattern p = mod.getPatterns().get(orders[i]);
            mod.getOrder().add(p);
        }

        // load samples
        for (int i = 0; i < 31; i++) {
            int sampleLen = mod.getSamples().get(i).getLength();
            byte[] sampleData = new byte[sampleLen];
            data.read(sampleData, 0, sampleLen);
            mod.getSamples().get(i).setData(sampleData);
        }
        return mod;
    }
}
