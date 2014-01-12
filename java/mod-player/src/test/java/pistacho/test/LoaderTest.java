/*
 * (c) 2013 Duncan Mac-Vicar P.
 *
 * Released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package pistacho.test;

import static org.junit.Assert.assertEquals;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.junit.runners.JUnit4;
import pistacho.Loader;
import pistacho.Module;
import pistacho.loader.MODLoader;
import pistacho.loader.S3MLoader;

@RunWith(JUnit4.class)
public class LoaderTest {

    @Test
    public void loadMODFile() throws Exception {
        Loader loader = new MODLoader();
        Module mod = loader.loadFile("/home/duncan/Downloads/mod.bobofmop.mod");
        assertEquals(4, mod.getNumberOfChannels());
        assertEquals("z.bob_of_mop", mod.getName());
        assertEquals(31, mod.getSamples().size());
        assertEquals("ST-04:heavybass14", mod.getSamples().get(5).getName());
        assertEquals(26, mod.getPatterns().size());
        assertEquals(30, mod.getOrder().size());

    }

    @Test
    public void loadS3MFile() throws Exception {
        Loader loader = new S3MLoader();
        Module mod = loader.loadFile("/home/duncan/Downloads/master.s3m");
        assertEquals(0, mod.getNumberOfChannels());
        assertEquals("Master Of Puppets".getBytes().length, mod.getName().getBytes().length);
    }
}
