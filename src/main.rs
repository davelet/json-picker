use app::channel;
use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};
use fltk::input::MultilineInput;
use fltk::table::Table;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut wind = Window::default().with_size(100, 200).center_screen().with_label("JSON HAND");
    let table = Table::default().with_label("table");
    table.
    let mut flex = Flex::default().with_size(120, 140).center_of_parent().column();
    flex.set_margin(30);
    let mut line1 = Flex::default().column();
    let input = MultilineInput::default().with_size(1, 2);
    let mut but_inc = Button::default().with_size(100,30).with_label("+");
    let mut frame = Frame::default().with_label("0");
    let mut but_dec = Button::default().with_label("-");
    line1.end();
    let mut line2 = Flex::default().column();
    let b = Button::default().with_label("2");
    line2.end();
    flex.end();
    table.end();
    wind.end();
    wind.make_resizable(true);
    wind.fullscreen(true);
    wind.show();
    /* previous counter code */
    let (s, r) = channel::<Message>();

    but_inc.emit(s, Message::Increment);
    but_dec.emit(s, Message::Decrement);

    while app.wait() {
        let label: i32 = frame.label().parse().unwrap();
        if let Some(msg) = r.recv() {
            match msg {
                Message::Increment => frame.set_label(&(label + 1).to_string()),
                Message::Decrement => frame.set_label(&(label - 1).to_string()),
            }
        }
    }
    app.run().unwrap();
}
#[derive(Copy, Clone)]
enum Message {
    Increment,
    Decrement
}