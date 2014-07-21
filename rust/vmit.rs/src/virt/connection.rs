extern crate std;
extern crate log;
extern crate debug;
use std::c_str::CString;
use super::ffi::*;
use super::Result;
use std::ptr::{mut_null};
use std::string::String;
use std::path::BytesContainer;

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


    pub fn get_sys_info(&mut self) -> String {
        let v =
        unsafe {
            CString::new(
                virConnectGetSysinfo(self.ptr, 0), true)
            .container_into_owned_bytes()
        };
        String::from_utf8(v).unwrap_or(String::new())
    }

}

impl Drop for Connection {
    fn drop(&mut self) {
        //println!("`Connection.drop()`: ptr={:?}", self.ptr);
        unsafe {
            virConnectClose(self.ptr);
        }
    }
}

#[cfg(test)]
mod test {

    use super::Connection;

    #[test]
    fn test_conn() {
        let mut conn = Connection::open("qemu+unix:///system").ok().unwrap();
        assert!(conn.get_sys_info().as_slice().ends_with("</sysinfo>\n"));
    }
}
