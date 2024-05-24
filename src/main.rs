use fltk::app;
use fltk::app::Scheme;
use fltk::prelude::{GroupExt, WidgetExt};

use crate::data::singleton::APP_WINDOW;
use crate::logic::handler::*;

mod component;
mod data;
mod logic;

fn main() {
    let mut app = app::App::default();
    app.set_scheme(Scheme::Plastic);

    let mut window = APP_WINDOW.lock().unwrap();
    let wind = window.get_window();
    wind.make_resizable(true);
    wind.show();

    window_resize(wind);
    listen_on_events(&app);
    make_ready();

    app.run().unwrap();
}
