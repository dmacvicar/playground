
use std::collections::HashMap;
use std::str;
use std::string::String;

fn build() {
    println!("build!");
}

fn main() {

    let mut cmd_map: HashMap<&str, fn() -> _> = HashMap::new();
    cmd_map.insert("build", build);

    let args = std::os::args().clone();

    for i in args.tail().iter() {
        println!("{}", i);
        match cmd_map.find_equiv(i) {
            Some(action) => {
                (*action)();
            }
            None => {
                println!("cmd extern?");
            }
        }
    }




}
