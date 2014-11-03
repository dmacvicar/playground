#![feature(unsafe_destructor)]
extern crate rustbox;
extern crate core;
use std::string::String;
use std::char::from_u32;
use std::vec::Vec;
use std::mem;

pub struct Size {
    width: uint,
    height: uint,
}

pub struct Pos {
    x: uint,
    y: uint,
}

trait Widget {
    fn get_min_size(&self) -> Size;
    /// set by the container
    fn set_size(&mut self, size: Size);

    fn draw(&mut self, pos: Pos, size: Size);
}

trait Container {
    fn add_widget<W: Widget + 'static>(&mut self, widget: W);
}

pub struct Button {
    message: String,
    size: Size,
}

impl Widget for Button {
    fn get_min_size(&self) -> Size {
        // text + border
        Size{width: self.message.len() + 2, height: 3}
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, pos: Pos, size: Size) {
        println!("draw button");
    }
}

pub struct HLayout {
    size: Size,
    widgets: Vec<Box<Widget + 'static>>,
}

impl Widget for HLayout {
    fn get_min_size(&self) -> Size {
        // horizontal is the sum of all
        // vertical is the biggest one
        // TODO
        Size{width: 0, height: 0}
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, pos: Pos, size: Size) {
        println!("draw hlayout");
    }

}

impl Container for HLayout {
    fn add_widget<W: Widget  + 'static>(&mut self, widget: W) {
        let owned = box widget;
        self.widgets.push(owned);
    }
}

pub struct Screen<'a> {
    widget: Option<Box<Widget + 'a>>
}

impl<'a> Screen<'a> {

    pub fn new() -> Screen<'a> {
        rustbox::init();
        rustbox::clear();

        // paint the bw blue
        for x in range(0, rustbox::width()) {
            for y in range(0, rustbox::height()) {
                rustbox::change_cell(x, y, ' ' as u32, 
                                     rustbox::convert_color(rustbox::White),
                                     rustbox::convert_color(rustbox::Blue));
            }
        }
        Screen {widget: None}
    }

    pub fn set_widget<W: Widget>(&mut self, widget: W) {
        let owned = box widget;
        self.widget = Some(owned);
    }

    pub fn wait() {
        rustbox::present();
        loop {
            match rustbox::poll_event() {
                rustbox::KeyEvent(_, _, ch) => {
                    match std::char::from_u32(ch) {
                        Some('q') => { break; },
                        _ => {}
                    }
                },
                _ => { }
            }
        }
    }
}

impl<'a> Widget for Screen<'a> {
    fn get_min_size(&self) -> Size {
        Size{width: rustbox::width(), height: rustbox::height()}
    }

    fn set_size(&mut self, size: Size) {
        // no nop
    }

    fn draw(&mut self, pos: Pos, size: Size) {
        println!("draw screen");
        match self.widget {
            Some(ref w) => println!("main widget"),
            None => println!("no op")
        }
    }
}

#[unsafe_destructor]
impl<'a> Drop for Screen<'a> {
    fn drop(&mut self) {
     rustbox::shutdown();
  }
}

