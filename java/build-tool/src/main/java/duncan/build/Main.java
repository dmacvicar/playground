package duncan.build;

import bsh.EvalError;
import bsh.Interpreter;
import bsh.NameSpace;
import bsh.This;

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
