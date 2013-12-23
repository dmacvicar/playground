package duncan.build;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Stack;
import java.nio.file.Paths;

import org.luaj.vm2.*;
import org.luaj.vm2.lib.jse.*;

import org.apache.tools.ant.BuildLogger;
import org.apache.tools.ant.DefaultLogger;
import org.apache.tools.ant.Project;
import org.apache.tools.ant.Target;
import org.apache.tools.ant.launch.Locator;
import org.apache.tools.ant.listener.AnsiColorLogger;
import org.apache.tools.ant.taskdefs.Delete;
import org.apache.tools.ant.taskdefs.Javac;
import org.apache.tools.ant.taskdefs.Mkdir;
import org.luaj.vm2.lib.OneArgFunction;
import org.luaj.vm2.lib.ZeroArgFunction;

public class Env implements ProjectBuilder {
     
    Project project = new Project();
    private Globals globals = JsePlatform.standardGlobals();   
    private LuaValue chunk;
    
    //BuildLogger logger = new DefaultLogger();
    BuildLogger logger = new AnsiColorLogger();
    String root;

    public Env(String root) {
        project.init();
        this.root = root;
        
        ProjectBuilder api = this;
        //globals.set("env", CoerceJavaToLua.coerce(api));
        globals.set("message", new message());
        globals.set("java", new java());
        
        logger.setOutputPrintStream(System.out);
        logger.setErrorPrintStream(System.out);
        logger.setMessageOutputLevel(Project.MSG_INFO);
        project.addBuildListener(logger);
        
        project.setBaseDir(new File(root));
    }
    
    public void run() throws Exception {
        
        // default name as the directory
        project.setName(Paths.get(root).getFileName().toString());

        Path buildFile = Paths.get(root, "build.lua");
        if (Files.exists(buildFile)) {
            chunk = globals.loadFile(buildFile.toString());
            chunk.call();
        }
        
        Target cleanTarget = new Target();
        cleanTarget.setProject(project);
        cleanTarget.setName("clean");
        cleanTarget.setDescription("Cleans the project");
        Delete del = new Delete();
        del.setDir(Paths.get(root, "build").toFile());
        del.setProject(project);
        del.setOwningTarget(cleanTarget);
        cleanTarget.addTask(del);
        project.addTarget("clean", cleanTarget);
        
        Target compileTarget = new Target();
        compileTarget.setProject(project);
        compileTarget.setName("compile");
        compileTarget.setDescription("Compiles the project");
        
        Mkdir mkdir = new Mkdir();
        mkdir.setDir(Paths.get(root, "build/classes").toFile());
        mkdir.setProject(project);
        mkdir.setOwningTarget(compileTarget);
        
        Javac javac = new Javac();
        javac.setDestdir(Paths.get(root, "build/classes").toFile());
        javac.setSrcdir(
                new org.apache.tools.ant.types.Path(
                        project, Paths.get(root, "/src/main/java").toString()));
        javac.setProject(project);
        javac.setOwningTarget(compileTarget);
        javac.setFork(true);
        javac.setIncludeantruntime(false);

        compileTarget.addTask(mkdir);
        compileTarget.addTask(javac);
        
        project.addTarget("compile", compileTarget);
        project.setDefault("compile");
        
        // apply a java configuration
        // TODO this should be a "plugin" later
        Path srcMainJava = Paths.get(root, "/src/main/java");
        if (Files.exists(srcMainJava)) {
           
        }

        project.setDefaultInputStream(null);
        
        project.executeTarget(project.getDefaultTarget());
        System.out.println(project.toString());
        
    }
    
    public void project(String name) {
        project.setName(name);
    }
    
    public void add_subdirectory(String dir) {
        
    }

    @Override
    public void java_library(String name) {
    }
    
    protected static class message extends OneArgFunction {
        public LuaValue call(LuaValue arg) {
            System.out.println(arg.tojstring());
            return LuaValue.NIL;
        }
    }

    protected static class java extends ZeroArgFunction {
        public LuaValue call() {
            System.out.println("Java!");
            return LuaValue.NIL;
        }
    }
}
