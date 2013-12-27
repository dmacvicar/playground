/*
 * (c) 2013 Duncan Mac-Vicar P.
 * released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package duncan.build;

import java.beans.BeanInfo;
import java.beans.IntrospectionException;
import java.beans.Introspector;
import java.beans.PropertyDescriptor;
import java.beans.Statement;
import java.lang.reflect.Method;
import org.luaj.vm2.LuaValue;
import org.luaj.vm2.Varargs;
import org.luaj.vm2.lib.OneArgFunction;
import org.luaj.vm2.lib.VarArgFunction;

public class LuaApi {

    /**
     * Takes all key/value pairs of a lua table and sets properties
     * of the same name of a bean with them.
     *
     * @param bean Bean to configure
     * @param data Lua table
     * @throws Exception
     */
    static void configureBeanWithTable(Object bean, LuaValue data) throws Exception {
        if (!data.istable()) {
            throw new Exception("Can't configure " + bean + ". " + data.tojstring() + " is not a table");
        }

        Class<?> klass = bean.getClass();
        try {
            BeanInfo info = Introspector.getBeanInfo(klass);
            PropertyDescriptor[] pds = info.getPropertyDescriptors();
            for (PropertyDescriptor pd : pds) {
                Method setter = pd.getWriteMethod();
                Class<?> propClass = pd.getClass();

                LuaValue luaProp = data.get(pd.getName());
                if (!luaProp.isnil()) {
                    if (luaProp.isstring() && pd.getPropertyType() == String.class) {
                        try {
                            Object[] setterArgs = new Object[1];
                            setterArgs[0] = luaProp.tojstring();
                            Statement stmt = new Statement(bean, setter.getName(), setterArgs);
                            stmt.execute();
                        } catch (Exception ex) {
                            throw new Exception("Can't configure " + bean + ": " + ex.getLocalizedMessage());
                        }
                    }
                }
            }
        } catch (IntrospectionException ex) {
             throw new Exception("Can't configure " + bean + ": " + ex.getLocalizedMessage());
        }

    }

    private abstract static class message extends OneArgFunction {
        protected Env env;
        public message(Env env) {
            this.env = env;
        }
        @Override
        public LuaValue call(LuaValue arg) {
            this.log(arg.tojstring());
            return LuaValue.NIL;
        }
        protected abstract void log(String message);
    }

    protected static class info extends message {
        public info(Env env) { super(env); }
        @Override
        protected void log(String message) {
            env.info(message);
        }
    }

    protected static class error extends message {
        public error(Env env) { super(env); }
        @Override
        protected void log(String message) {
            env.error(message);
        }
    }

    protected static class warn extends message {
        public warn(Env env) { super(env); }
        @Override
        protected void log(String message) {
            env.warn(message);
        }
    }

    /**
     * Configures a project with a plugin
     *
     * Eg: apply(java)
     */
    protected static class apply extends VarArgFunction {
        private final Env env;
        public apply(Env env) {
            this.env = env;
        }

        @Override
        public LuaValue invoke(Varargs args) {
            if (args.narg() < 1) {
                env.error("apply requires plugin name at least...");
                return LuaValue.NIL;
            }

            LuaValue luaPlugin = args.arg(1);
            Plugin plugin = null;
            if (luaPlugin.isuserdata(Plugin.class)) {
                plugin = (Plugin) luaPlugin.touserdata(Plugin.class);
            }
            else {
                env.error("plugin is a " + luaPlugin.typename());
                return LuaValue.NIL;
            }

            LuaValue data = args.arg(2);
            try {
                configureBeanWithTable(plugin, data);
            }
            catch (Exception e) {
                env.error("Can't configure " + plugin + ": " + e.getLocalizedMessage());
            }

            plugin.configure(env.getProject());
            return LuaValue.NIL;
        }
    }
}
