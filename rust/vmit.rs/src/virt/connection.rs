extern crate log;
extern crate debug;
mod ffi;
mod types;
use ffi::*;


pub struct Connection {
    ptr: virConnectPtr,
}

impl Connection {

    fn from_ptr(ptr: virConnectPtr) -> Connection {
        Connection{ptr: ptr}
    }

    fn open(name: &str) -> VirResult<Connection> {
        unsafe {
            let r = name.with_c_string(|_name| {
                match virConnectOpen(_name) {
                    ptr::mut_null => Err("shit!"),
                    p => Ok(Connection::from_ptr(p))
                }
            });
            return r;
        };
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("`Connection.drop()`: ptr={:?}", self.ptr);
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
        let conn = Connection::open("qemu://system");
        assert_eq!("hello", "hell");
    }
}
