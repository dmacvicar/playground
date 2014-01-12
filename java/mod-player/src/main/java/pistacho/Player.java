/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

package pistacho;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Platform;
import com.sun.jna.Pointer;

/**
 *
 * @author duncan
 */
public class Player {

    public static interface Xmp extends Library {
        Pointer xmp_create_context();
        int xmp_load_module(Pointer ctx, String path);
        void xmp_free_context(Pointer ctx);
    }


    public static void main(String[] args) throws Exception {
        System.out.println("Play!");
        Xmp xmp = (Xmp) Native.loadLibrary("xmp", Xmp.class);
        Pointer ctx = xmp.xmp_create_context();
        int ret = xmp.xmp_load_module(ctx, "/home/duncan/Downloads/master.s3m");
        if (ret != 0) {
            throw new Exception("Fail!");
        }
        
            
        
        xmp.xmp_free_context(ctx);
    }
}
