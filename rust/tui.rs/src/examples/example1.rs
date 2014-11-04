
extern crate tui;
use tui::Size;

fn main() {
    let mut screen = tui::Screen::new();

    let btn = tui::Button::new(String::from_str("Hello"), Size::new(10, 10));
    screen.set_widget(btn);
    screen.draw();
    screen.wait();
}
