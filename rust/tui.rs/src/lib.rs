#![feature(globs)]
#![feature(unsafe_destructor)]
extern crate ncurses;
extern crate core;
use std::string::String;
use std::char::from_u32;
use std::vec::Vec;
use std::mem;

use ncurses::*;

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
    fn get_preferred_size(&self) -> Size;
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
    fn get_preferred_size(&self) -> Size {
        // [ Text ]
        Size{width: self.message.len() + 4, height: 1}
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&self, pos: Pos, size: Size) {
        let pref = self.get_preferred_size();
        if size.width >= pref.width && size.height >= pref.height {
            let mut x = pos.x;
            let y = pos.y;
            mv(y as i32, x as i32);
            addstr("[ ");
            x = x + 2; mv(y as i32, x as i32);
            addstr(self.message.as_slice());
            x = x + self.message.len();
            addstr(" ]");
        }
    }
}

pub struct HLayout {
    size: Size,
    widgets: Vec<Box<Widget + 'static>>,
}

impl Widget for HLayout {
    fn get_preferred_size(&self) -> Size {
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
        initscr();
        raw();
        keypad(stdscr, true);
        noecho();
        start_color();
        curs_set(CURSOR_INVISIBLE);
        refresh();

        Screen {widget: None}
    }

    pub fn set_widget<W: Widget + 'a>(&mut self, widget: W) {
        let owned = (box widget) as Box<Widget>;
        self.widget = Some(owned);
    }

    pub fn wait(&self) {
        refresh();
        let ch = getch();
        loop {
            match ch {
                KEY_F1 => { break; },
                _ => { }
            }
        }
    }

    pub fn draw(&self) {
        // screen size
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr, &mut max_y, &mut max_x);
        match self.widget {
            Some(ref w) => w.draw(Pos{x: 0, y: 0},
                                  Size{width: max_x as uint, height: max_y as uint}),
            None => println!("no op")
        }
    }
}

#[unsafe_destructor]
impl<'a> Drop for Screen<'a> {
    fn drop(&mut self) {
     endwin();
  }
}

