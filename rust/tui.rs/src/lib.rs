extern crate rustbox;
use std::string::String;

pub struct Button {
    msg: String
}

pub struct Screen;

impl Screen {

    pub fn init() {
        rustbox::init();
    }
}
