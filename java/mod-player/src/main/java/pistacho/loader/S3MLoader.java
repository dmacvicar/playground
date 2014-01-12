/*
 * (c) 2013 Duncan Mac-Vicar P.
 *
 * Released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package pistacho.loader;

import java.io.InputStream;
import java.nio.charset.Charset;
import pistacho.LoaderBase;
import pistacho.Module;

/**
 * Format documentation
 * http://schismtracker.org/wiki/FS3MDOC.TXT
 */
public class S3MLoader extends LoaderBase {

    @Override
    public Module loadStream(InputStream data) throws Exception {
        Module mod = new Module();
        mod.setName(readNullTerminatedString(data, 28, Charset.defaultCharset()));
            byte[] title = new byte[28];
            //int sig = data.readByte() & 0xFF;
            //System.out.println(String.format("0x%2s", Integer.toHexString(sig)).replace(' ', '0'));
            return mod;
    }
}
