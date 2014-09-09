extern crate rustbox;
use std::string::String;
use std::char::from_u32;
use std::vec::Vec;

pub struct Size {
    width: uint,
    height: uint,
}

trait Widget {
    fn get_min_size(&self) -> Size;
}

trait Painter<'a> {
    fn new(&'a Widget) -> Self;
}

pub struct ConsolePainter<'a> {
    widget: &'a Widget + 'a,
}

impl<'a> Painter<'a> for ConsolePainter<'a> {
    fn new(widget: &'a Widget) -> ConsolePainter {
        ConsolePainter{widget: widget}
    }

}

pub struct Button {
    message: String
}

impl Widget for Button {
    fn get_min_size(&self) -> Size {
        // text + border
        Size{width: self.message.len() + 2, height: 3}
    }
}

pub struct HLayout;

impl Widget for HLayout {
    fn get_min_size(&self) -> Size {
        // horizontal is the sum of all
        // vertical is the biggest one
        // TODO
        Size{width: 0, height: 0}
    }
}

pub struct Screen;

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

