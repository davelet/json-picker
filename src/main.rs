use component::window::AppWindow;
use fltk::app::Scheme;
use fltk::app;
use fltk::prelude::{GroupExt, WidgetExt};

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


