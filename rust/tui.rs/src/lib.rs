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

impl Size {
    pub fn new(width: uint, height: uint) -> Size {
        Size{width: width, height: height}
    }
}

pub struct Pos {
    x: uint,
    y: uint,
}

pub trait Widget {
    fn get_min_size(&self) -> Size;
    /// set by the container
    fn set_size(&mut self, size: Size);

    fn draw(&self, pos: Pos, size: Size);
}

pub trait Container {
    fn add_widget<W: Widget + 'static>(&mut self, widget: W);
}

pub struct Button {
    message: String,
    size: Size,
}

impl Button {
    pub fn new(message: String, size: Size) -> Button {
        Button{message: message, size: size}
    }
}

impl Widget for Button {
    fn get_min_size(&self) -> Size {
        // text + border
        Size{width: self.message.len() + 2, height: 3}
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&self, pos: Pos, size: Size) {
        for x in range(pos.x, size.width) {
            for y in range(pos.y, size.height) {
                rustbox::change_cell(x, y, ' ' as u32, 
                                     rustbox::convert_color(rustbox::White),
                                     rustbox::convert_color(rustbox::White));
            }
        }
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

    fn draw(&self, pos: Pos, size: Size) {
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

        Screen {widget: None}
    }

    pub fn set_widget<W: Widget + 'a>(&mut self, widget: W) {
        let owned = (box widget) as Box<Widget>;
        self.widget = Some(owned);
    }

    pub fn wait(&self) {
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

    pub fn draw(&self) {
        match self.widget {
            Some(ref w) => w.draw(Pos{x: 0, y: 0},
                                  Size{width: rustbox::width(), height: rustbox::height()}),
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

