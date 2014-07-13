extern crate term;

pub fn say(msg: &str, color: term::color::Color) {
    let mut t = term::stdout().unwrap();
    t.fg(color).unwrap();

    t.write_line(msg).unwrap();
    t.reset().unwrap();
}

pub fn say_error(msg: &str) {
    say(msg, term::color::RED);
}
