extern crate std;
extern crate log;
extern crate debug;
use std::c_str::CString;
use super::ffi::*;
use super::Result;
use std::ptr::{mut_null};
use std::string::String;
use std::path::BytesContainer;
use std::fmt::Formatter;
use std::fmt::Result;

pub struct Domain {
    ptr: virDomainPtr,
}

impl Domain {

    pub fn from_ptr(ptr: virDomainPtr) -> Domain {
        Domain{ptr: ptr}
    }

    /// http://libvirt.org/html/libvirt-libvirt.html#virDomainGetName
    pub fn get_name(&self) -> String {
        String::from_utf8(
            unsafe {
                CString::new(
                virDomainGetName(self.ptr), false)
                    .container_into_owned_bytes()
            }).unwrap_or(String::new())
    }
}

impl Drop for Domain {
    fn drop(&mut self) {
        //println!("`Domain.drop()`: ptr={:?}", self.ptr);
        unsafe {
            virDomainFree(self.ptr);
        }
    }
}

impl std::fmt::Show for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

#[cfg(test)]
mod test {

    use super::Domain;

    #[test]
    fn test_conn() {
        //let mut conn = Domain::open("qemu+unix:///system").ok().unwrap();
        //assert!(conn.get_sys_info().as_slice().ends_with("</sysinfo>\n"));
    }
}
