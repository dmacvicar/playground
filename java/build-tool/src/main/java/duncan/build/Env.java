/*
 * (c) 2013 Duncan Mac-Vicar P.
 * released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package duncan.build;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

import org.luaj.vm2.*;
import org.luaj.vm2.lib.jse.*;

import org.apache.tools.ant.BuildLogger;
import org.apache.tools.ant.listener.AnsiColorLogger;

public class Env implements Api {

    Project project;
    private final Globals globals;
    private LuaValue chunk;

    //BuildLogger logger = new DefaultLogger();
    BuildLogger logger = new AnsiColorLogger();

    public Env(String root) {
        this.globals = JsePlatform.standardGlobals();
        project = new Project(root);
        JavaPlugin java = new JavaPlugin();
        globals.set("java",  CoerceJavaToLua.coerce(java));
        globals.set("project",  CoerceJavaToLua.coerce(project));

        globals.set("info", new LuaApi.info(this));
        globals.set("error", new LuaApi.error(this));
        globals.set("warn", new LuaApi.warn(this));
        globals.set("apply", new LuaApi.apply(this));

        logger.setOutputPrintStream(System.out);
        logger.setErrorPrintStream(System.out);
        logger.setMessageOutputLevel(org.apache.tools.ant.Project.MSG_INFO);
        project.getAntProject().addBuildListener(logger);
    }

    @Override
    public Project getProject() {
        return this.project;
    }

    @Override
    public void info(String message) {
        this.project.getAntProject().log(message, org.apache.tools.ant.Project.MSG_INFO);
    }

    @Override
    public void warn(String message) {
        this.project.getAntProject().log(message, org.apache.tools.ant.Project.MSG_WARN);
    }

    @Override
    public void error(String message) {
        this.project.getAntProject().log(message, org.apache.tools.ant.Project.MSG_ERR);
    }

    public void run() throws Exception {
        Path buildFile = Paths.get(project.getRoot(), "build.lua");
        if (Files.exists(buildFile)) {
            project.log("Executing build script: " + buildFile.toString());
            chunk = globals.loadFile(buildFile.toString());
            LuaValue ret = chunk.call();
        }
        else {
            project.log("No build.lua script found");
        }

        org.apache.tools.ant.Project ant = project.getAntProject();
        ant.executeTarget(ant.getDefaultTarget());
    }
}
