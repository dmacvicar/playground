
use std::gc::GC;

struct Foo;

impl Foo {
    fn hello(&self) {
        println!("hello!");
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    let _x = box Foo;
    let _gced = box(GC) Foo;

    let other_x = _gced;

    _gced.hello();

    _x.hello();
}
