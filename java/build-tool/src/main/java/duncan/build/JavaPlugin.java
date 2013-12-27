/*
 * (c) 2013 Duncan Mac-Vicar P.
 * released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package duncan.build;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import org.apache.tools.ant.Target;
import org.apache.tools.ant.taskdefs.Delete;
import org.apache.tools.ant.taskdefs.Javac;
import org.apache.tools.ant.taskdefs.Mkdir;

public class JavaPlugin implements Plugin {

    public JavaPlugin() {
    }

    private static final String DEFAULT = "main";

    private String fakeProperty;

    public void setFakeProperty(String prop) {
        System.out.println("Set Fake to  " + prop);
        this.fakeProperty = prop;
    }

    @Override
    public void configure(Project p) {
        // apply a java configuration
        Path srcMainJava = Paths.get(p.getRoot(), "/src/main/java");
        if (Files.exists(srcMainJava)) {
           configureSourceSet(p, DEFAULT);
        }
        else {
            p.log("Can't apply java configuration without /src/main/java");
        }
    }

    private void configureSourceSet(Project p, String name) {
        p.log("Configuring java target set for '" + name + "'");
        org.apache.tools.ant.Project ant = p.getAntProject();

        Target cleanTarget = new Target();
        cleanTarget.setProject(ant);
        cleanTarget.setName("clean");
        cleanTarget.setDescription("Cleans the project");

        Delete del = new Delete();
        del.setDir(Paths.get(p.getRoot(), "build").toFile());
        del.setProject(p.getAntProject());
        del.setOwningTarget(cleanTarget);
        cleanTarget.addTask(del);
        ant.addTarget("clean", cleanTarget);

        Target compileTarget = new Target();
        compileTarget.setProject(ant);
        compileTarget.setName("compile");
        compileTarget.setDescription("Compiles the project");

        Mkdir mkdir = new Mkdir();
        mkdir.setDir(Paths.get(p.getRoot(), "build/classes").toFile());
        mkdir.setProject(ant);
        mkdir.setOwningTarget(compileTarget);

        Javac javac = new Javac();
        javac.setDestdir(Paths.get(p.getRoot(), "build/classes").toFile());
        javac.setSrcdir(
                new org.apache.tools.ant.types.Path(
                        ant, Paths.get(p.getRoot(), "/src/main/java").toString()));
        javac.setProject(ant);
        javac.setOwningTarget(compileTarget);
        javac.setFork(true);
        javac.setIncludeantruntime(false);

        compileTarget.addTask(mkdir);
        compileTarget.addTask(javac);

        ant.addTarget("compile", compileTarget);
        ant.setDefault("compile");
    }
}
