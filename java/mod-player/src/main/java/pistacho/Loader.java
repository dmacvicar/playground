/*
 * (c) 2013 Duncan Mac-Vicar P.
 *
 * Released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package pistacho;

import java.io.InputStream;

public interface Loader {
    public Module loadStream(InputStream data) throws Exception;
    public Module loadFile(String path) throws Exception;
}
