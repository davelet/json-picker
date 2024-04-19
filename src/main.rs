use app::channel;
use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};
fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default().with_size(160, 200).center_screen().with_label("Counter");
    let flex = Flex::default().with_size(120, 140).center_of_parent().column();
    let mut but_inc = Button::default().with_label("+");
    let mut frame = Frame::default().with_label("0");
    let mut but_dec = Button::default().with_label("-");
    flex.end();
    wind.end();
    wind.make_resizable(true);
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