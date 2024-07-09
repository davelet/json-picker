use fltk::app;
use fltk::app::Scheme;
use fltk::prelude::{GroupExt, WidgetExt};
use fltk_theme::{color_themes, ColorTheme};

use crate::data::singleton::{APP_WINDOW, LOADING_WINDOW};
use crate::logic::handler::*;

mod component;
mod data;
mod logic;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");
    let mut app = app::App::default();
    app.set_scheme(Scheme::Plastic);
    let theme = ColorTheme::new(color_themes::GRAY_THEME);
    theme.apply();
    {
        let mut window = APP_WINDOW.lock().unwrap();
        let wind = window.get_window();
        wind.make_resizable(true);

        let mut loading = LOADING_WINDOW.lock().unwrap();
        loading.pin(app::screen_size());
        loading.get().show();

        // wind.show();
    }

    handle_event(&app);
}