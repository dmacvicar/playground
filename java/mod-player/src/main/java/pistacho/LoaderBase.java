/*
 * (c) 2013 Duncan Mac-Vicar P.
 *
 * Released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package pistacho;

import java.io.BufferedInputStream;
import java.io.FileInputStream;
import java.io.InputStream;
import java.nio.charset.Charset;

public abstract class LoaderBase implements Loader {

    @Override
    public Module loadFile(String path) throws Exception {
        return loadStream(new BufferedInputStream(new FileInputStream(path)));
    }

    protected String readNullTerminatedString(InputStream is, int max, Charset charSet) throws Exception {
        final byte[] arr = new byte[max];
        is.read(arr, 0, max);
        return readNullTerminatedString(arr, charSet);
    }

    private String readNullTerminatedString(byte[] arr, Charset charSet) throws Exception {
        int i;
        for (i = 0; i < arr.length && arr[i] != 0; i++) {
        }
        String str = new String(arr, 0, i, charSet);
        return str;
    }

    protected int readWord(InputStream is) throws Exception {
        int hi = is.read();
        int lo = is.read();
        return (hi * 0x100 + lo) * 2;
    }
}
