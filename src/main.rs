use fltk::app;
use fltk::app::Scheme;
use fltk::prelude::{GroupExt, WidgetExt};
use component::window::AppWindow;
use crate::data::notify_enum::NotifyType;

mod component;
mod logic;
mod data;

fn main() {
    let mut app = app::App::default();
    app.set_scheme(Scheme::Plastic);
    let (s, r) = app::channel::<NotifyType>();

    let wind = AppWindow::new(s.clone(), r.clone());
    let mut wind = wind.get_window();
    wind.make_resizable(true);
    wind.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                NotifyType::Input(s) => println!("{}", s),
                _ => (), // 什么都不做
            }
        }
    }

    app.run().unwrap();
}


