
use types::*;

#[link(name = "virt")]
extern {

    pub fn virConnectOpen(name: *const c_char) -> virConnectPtr;
    pub fn virConnectClose(conn: virConnectPtr);
}

