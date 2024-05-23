use fltk::app;
use fltk::app::Scheme;
use fltk::prelude::{GroupExt, WidgetExt};

use data::constants::START_TIMEOUT;
use data::notify_enum::{ComputeStatus, NotifyType};

use crate::data::singleton::{APP_WINDOW, CHANNEL};

mod component;
mod data;
mod logic;

fn main() {
    let mut app = app::App::default();
    app.set_scheme(Scheme::Plastic);

    let mut window = APP_WINDOW.lock().unwrap();
    let mut wind = window.get_window();
    wind.make_resizable(true);
    wind.show();

    app::add_timeout3(START_TIMEOUT, |_| {CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Ready));} );
    let ar = app.run();
    if let Ok(_) = ar {
        
    }
}
