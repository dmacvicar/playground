extern crate libc;
use self::libc::{c_void, c_char, c_int};

//pub type virConnectPtr = *const c_void;
pub type virConnectPtr = *mut c_void;

#[link(name = "virt")]
extern {

    pub fn virConnectOpen(name: *const c_char) -> virConnectPtr;
    pub fn virConnectClose(conn: virConnectPtr);

    pub fn virConnectGetSysinfo(conn: virConnectPtr, flags: c_int) -> *const c_char;

}

