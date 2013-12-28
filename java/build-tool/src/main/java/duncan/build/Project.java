/*
 * (c) 2013 Duncan Mac-Vicar P.
 * released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package duncan.build;

import java.io.File;
import java.nio.file.Paths;

public class Project {
    final private String root;

    org.apache.tools.ant.Project antProject;

    public Project(String root) {
        antProject = new org.apache.tools.ant.Project();
        antProject.init();
        antProject.setBaseDir(new File(root));

        // default name as the directory
        antProject.setName(Paths.get(root).getFileName().toString());

        this.root = root;
    }

    public String getRoot() {
        return this.root;
    }

    public void log(String message) {
        antProject.log(message);
    }

    public org.apache.tools.ant.Project getAntProject() {
        return antProject;
    }

    public String getName() {
        return antProject.getName();
    }

    public void setName(String name) {
        antProject.setName(name);
    }

    public void apply(Plugin plugin) {
        plugin.configure(this);
    }
}
