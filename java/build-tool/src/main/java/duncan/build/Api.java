/*
 * (c) 2013 Duncan Mac-Vicar P.
 * released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package duncan.build;

/**
 *
 * @author duncan
 */
public interface Api {
    public Project getProject();

    public void info(String message);

    public void warn(String message);

    public void error(String message);
}
