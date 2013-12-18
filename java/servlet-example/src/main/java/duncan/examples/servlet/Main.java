/*
 * Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/
 */
package duncan.examples.servlet;

import org.eclipse.jetty.server.Server;
import org.eclipse.jetty.servlet.ServletContextHandler;
import org.eclipse.jetty.servlet.ServletHolder;

/**
 *
 * @author Duncan Mac-Vicar P.
 */
public class Main {
    private static final int PORT = 8081;

    public static void main(String[] args) {
        Server server = new Server(PORT);
        ServletContextHandler context =
                new ServletContextHandler(ServletContextHandler.SESSIONS);
        server.setHandler(context);

        context.addServlet(new ServletHolder(new HelloWorldServlet()), "/*");
        server.setHandler(context);
        try {
            server.start();
            server.join();
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }
}
