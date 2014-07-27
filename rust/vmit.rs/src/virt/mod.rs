extern crate std;

pub use self::connection::{Connection};
pub use self::domain::{Domain};
pub type Result<T> = std::result::Result<T, &'static str>;

mod connection;
mod domain;
mod ffi;

#[cfg(test)]
mod test {

    use super::Connection;
    use super::Domain;

    #[test]
    fn test_conn_testdrv() {
        let mut ret = Connection::open("test:///default");
        match ret {
            Ok(conn) => {}
            Err(msg) => fail!(msg)
        }
    }

    #[test]
    fn test_list_domains() {
        let mut conn = Connection::open("test:///default").ok().unwrap();
        match conn.get_all_domains() {
            Ok(domains) => {
                assert_eq!(domains.len(), 1);
                let first = domains.iter().next();
                match first {
                    Some(dom) => {
                        assert_eq!(dom.get_name().as_slice(), "test")
                    },
                    _ => fail!("no domains")
                }
            },
            Err(msg) => {
                fail!("There should be at least one domain")
            }
        }
    }
}
