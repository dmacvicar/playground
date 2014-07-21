#![crate_type = "lib"]
#![crate_name = "vmit"]
#![feature(globs)]

use std::os::getcwd;
use std::owned::Box;

pub mod virt;

pub mod cli;

pub struct Workspace {
    dir: Box<Path>
}

impl Workspace {

    pub fn new(dir: &Path) -> Workspace {
        Workspace { dir: box dir.clone()}
    }

    pub fn from_pwd() -> Workspace {
        Workspace::new(&std::os::getcwd())
    }
}

impl std::fmt::Show for Workspace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Hi: {}", self.dir.display())
    }
}

#[cfg(test)]
mod test {

    use std::io::TempDir;
    use super::Workspace;

    #[test]
    fn test_workspace() {
        match TempDir::new("vmit") {
            Some(tmpdir) => {
                println!("{}", tmpdir.path().display());
                let ws = Workspace::new(tmpdir.path()); 

            }
            None => {
              fail!("ups!")
            }
        }
    }
}

