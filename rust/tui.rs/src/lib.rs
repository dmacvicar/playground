extern crate rustbox;
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
}

trait Container {
    //fn get_children(&self) -> &[Widget];

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
}

impl Container for HLayout {
    fn add_widget<W: Widget  + 'static>(&mut self, widget: W) {
        let owned = box widget;
        self.widgets.push(owned);
    }
}

pub struct Screen;

impl Widget for Screen {
    fn get_min_size(&self) -> Size {
        Size{width: rustbox::width(), height: rustbox::height()}
    }

    fn set_size(&mut self, size: Size) {
        // no nop
    }
}

impl Screen {

    pub fn init() {
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

impl Drop for Screen {
  fn drop(&mut self) {
     rustbox::shutdown();
  }
}

