use fltk::app;
use fltk::app::Scheme;
use fltk::prelude::{GroupExt, WidgetExt};
use component::window::AppWindow;

mod component;
mod logic;
mod data;

fn main() {
    let mut app = app::App::default();
    app.set_scheme(Scheme::Plastic);

    let wind = AppWindow::new();
    let mut wind = wind.get_window();
    wind.make_resizable(true);
    wind.show();

    app.run().unwrap();
}


