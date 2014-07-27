extern crate libc;
use self::libc::{c_void, c_char, c_int, c_uint};

pub type virConnectPtr = *mut c_void;
pub type virDomainPtr = *mut c_void;

#[link(name = "virt")]
extern {

    // Connection
    pub fn virConnectOpen(name: *const c_char) -> virConnectPtr;
    pub fn virConnectClose(conn: virConnectPtr);

    pub fn virConnectGetSysinfo(conn: virConnectPtr, flags: c_int) -> *const c_char;

    pub fn virConnectListAllDomains(conn: virConnectPtr,
                                    domains: *mut *mut virDomainPtr, flags: c_uint) -> c_uint;

    // Dommain
    pub fn virDomainGetName(domain: virDomainPtr) -> *const c_char;
    pub fn virDomainFree(domain: virDomainPtr) -> c_int;
}

