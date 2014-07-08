
use std::collections::HashMap;
use std::str;

fn build() {
    println!("build!"); 
}

fn main() {

    let mut cmd_map: HashMap<&str, ||> = HashMap::new();
    cmd_map.insert("build", build);

    let args = std::os::args().clone();

    for i in args.iter() {
        println!("{}", i);
        match cmd_map.find(i) {
            Some(action) => {
                action();
            }
            None => {
                println!("cmd extern?");
            }
        }
    }




}
