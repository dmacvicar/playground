
extern crate vmit;
use vmit::Workspace;

fn main() {
    let ws = Workspace::from_pwd();
    println!("{}", ws);
}
