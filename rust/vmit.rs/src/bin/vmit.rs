extern crate getopts;
extern crate vmit;

use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;
use std::vec::Vec;
use std::string::String;
use std::iter::FromIterator;
use std::io::stdio::stdout;
use std::io::Writer;

use vmit::Workspace;
use vmit::cli;
use vmit::virt;

fn do_work(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-o\t\tOutput");
    println!("-h --help\tUsage");
}

fn main() {

    let mut conn = virt::Connection::open("qemu+unix:///system").ok().unwrap();
    println!("{}", conn.get_sys_info());


    cli::say_error("vmit starting");

    let subcmds = ["init"];

    let args: Vec<String> = os::args();

    let program = args.get(0).clone();

    let global_args = args
        .iter().skip(1)
        .map(|s| s.clone())
        .take_while(|x| subcmds.iter().any(|&y| y != x.as_slice()))
        .collect::<Vec<String>>();

    let subcmd_args = args
        .iter().skip(1)
        .map(|s| s.clone())
        .skip_while(|x| subcmds.iter().any(|&y| y != x.as_slice()))
        .collect::<Vec<String>>();

    println!("{}", global_args);
    println!("{}", subcmd_args);

    let opts = [
        optopt("o", "", "set output file name", "NAME"),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(global_args.as_slice(), opts) {
        Ok(m) => { m }
        Err(f) => {
            fail!(f);
        }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        (*matches.free.get(0)).clone()
    } else {
        print_usage(program.as_slice(), opts);
        return;
    };
    do_work(input.as_slice(), output);
}
