/*
 * Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/
 */
package duncan.examples.servlet;

import org.eclipse.jetty.server.Server;
import org.eclipse.jetty.servlet.ServletContextHandler;
import org.eclipse.jetty.servlet.ServletHolder;
import org.eclipse.jetty.servlet.FilterHolder;

import com.opensymphony.sitemesh.webapp.SiteMeshFilter;
import java.util.EnumSet;
import javax.servlet.DispatcherType;
import javax.servlet.FilterConfig;

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
        context.addFilter(new FilterHolder(SiteMeshFilter.class), "/*", EnumSet.of(DispatcherType.REQUEST));

        SiteMeshFilter filter = new SiteMeshFilter();
        filter.init(null);
        SiteMeshFilterBuilder
        
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
