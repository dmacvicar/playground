extern crate std;
extern crate log;
extern crate debug;
extern crate libc;
use std::c_str::CString;
use super::ffi::*;
use super::Result;
use super::Domain;
use std::ptr::{mut_null};
use std::string::String;
use std::path::BytesContainer;
use std::vec::Vec;

pub struct Connection {
    ptr: virConnectPtr,
}

impl Connection {

    fn from_ptr(ptr: virConnectPtr) -> Connection {
        Connection{ptr: ptr}
    }

    pub fn open(name: &str) -> Result<Connection> {
            let r = name.with_c_str(|_name| {
                let p = unsafe { virConnectOpen(_name) };
                if p.is_null() {
                    Err("shit!")
                }
                else {
                    Ok(Connection::from_ptr(p))
                }
            });
            return r;
    }

    /// http://libvirt.org/html/libvirt-libvirt.html#virConnectGetsysinfo
    pub fn get_sys_info(&mut self) -> String {
        String::from_utf8(
            unsafe {
                CString::new(
                    virConnectGetSysinfo(self.ptr, 0), true)
                    .container_into_owned_bytes()
            }).unwrap_or(String::new())
    }

    pub fn get_all_domains(&mut self) -> Result<Vec<Domain>> {
        unsafe {
            let mut cdomains: *mut virDomainPtr = mut_null();
            let flags = 0;
            let ret = virConnectListAllDomains(self.ptr, &mut cdomains, flags);
            if ret < 0 {
                return Err("Shit!");
            }

            let domains = Vec::from_fn(ret as uint, |idx| Domain::from_ptr(*cdomains.offset(idx as int)));

            libc::free(*cdomains);
            Ok(domains)
        }
    }

}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            virConnectClose(self.ptr);
        }
    }
}

