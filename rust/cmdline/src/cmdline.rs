
extern crate getopts;

use std::collections::HashMap;
use std::str;
use std::string::String;
use std::os;
use getopts::{optopt,optflag,getopts,OptGroup};

fn build() {
    println!("build!");
}

struct Command {

    /// internal subcommands
    isubcmds: HashMap<String, fn()>,
}

impl Command {

    fn new() -> Command {
        Command {
            isubcmds: HashMap::new(),
        }
    }

    /// adds a subcommand
    fn add_subcmd(&mut self, name: &str, func: fn()) {
        self.isubcmds.insert(String::from_str(name), func);
    }

}


fn main() {

    let mut cmd_map: HashMap<&str, fn() -> _> = HashMap::new();
    cmd_map.insert("build", build);

    let args = std::os::args().clone();

    for cmdname in args.tail().iter() {
        println!("{}", cmdname);
        match cmd_map.find_equiv(cmdname) {
            Some(action) => {
                (*action)();
            },
            None => {
                // external command
                let extcmd_path: Option<Path> = match os::self_exe_path() {
                    Some(exe_path) => {
                        // now we need the name of the current program
                        let this_prog = args.get(0);
                        Some(exe_path.join(Path::new(format!("{}-{}", this_prog, cmdname))))
                    },
                    None => {
                        fail!("Impossible to fetch the path of this executable.");
                        let ret: Option<Path> = None;
                        ret
                    },
                };

                match extcmd_path {
                    Some(path) => println!("about to execute {}", path.display()),
                    None => {
                        fail!("command not known {}", cmdname);
                    }
                }
                //

                //println!("{}", os::self_exe_path());
                /*
                let command = format!("{}-{}", cmd);
                let mut command = match os::self_exe_path() {
                    Some(path) => Command::new(path.join(command)),
                    None => Command::new(command),
                };
                println!("cmd extern?");
                */
            },
        }
    }




}
