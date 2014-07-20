extern crate std;

pub use self::connection::{Connection};
pub type Result<T> = std::result::Result<T, &'static str>;

mod connection;
mod ffi;

