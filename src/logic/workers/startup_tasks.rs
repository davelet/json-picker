use crate::{data::singleton::APP_WINDOW, logic::app_startup::load_location};
use fltk::prelude::{DisplayExt, WidgetExt};
use crate::data::singleton::{ACTION_BTNS, JSON_INPUT_BOX};
use crate::logic::app_startup::load_snapshot;

pub(crate) trait StartupTask {
    fn new() -> Self;
    fn execute(&mut self);
}

pub(crate) struct AppWindowLocationLoadTask;

impl StartupTask for AppWindowLocationLoadTask {
    fn new() -> Self {
        Self
    }

    fn execute(&mut self) {
        let data = load_location();
        if let Some((x, y, w, h)) = data {
            let mut window = APP_WINDOW.lock().unwrap();
            let wind = window.get_window();
            wind.resize(
                x as i32,
                y as i32,
                w as i32,
                h as i32,
            );
        }

        let json = load_snapshot();
        if let Some(json) = json {
            {
                let inbox = JSON_INPUT_BOX.lock().unwrap();
                let mut buffer = inbox.buffer().unwrap();
                buffer.set_text(&json);
            }
            {
                let mut btns = ACTION_BTNS.lock().unwrap();
                let parse_btn = &mut btns[0];
                // parse_btn.do_callback();
                let _ = parse_btn.take_focus();
            }
        }
    }
}
