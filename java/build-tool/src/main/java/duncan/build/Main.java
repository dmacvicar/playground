/*
 * (c) 2013 Duncan Mac-Vicar P.
 * released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package duncan.build;

public class Main {

    public static void main(String[] args) {

        //String root = args.length > 0 ? args[0] : System.getProperty("user.dir");
        String root = "/space/git/playground/java/build-tool/src/test/resources/helloworld-explicit";
        Env env = new Env(root);
        try {
            env.run();
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

}
