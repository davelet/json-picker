use std::rc::Rc;
use fltk::{enums::Event, prelude::{GroupExt, WidgetBase, WidgetExt}, window::{self, Window}};
use fltk::app::Sender;
use fltk::prelude::WindowExt;

use crate::data::{DEFAULT_HEIGHT, DEFAULT_WIDTH, GLOBAL_CHANNEL, MIN_HEIGHT, MIN_WIDTH};
use crate::data::notify_enum::NotifyType;

use super::border_panel::WholeViewPanel;

pub(crate) struct AppWindow {
    window: Window
}

impl AppWindow {
    pub(crate) fn new() -> Self{
        let (width, height) = (DEFAULT_WIDTH, DEFAULT_HEIGHT);

        let mut wind = window::Window::default()
            .with_size(width, height)
            .with_label("JSON HAND");
        wind.size_range(MIN_WIDTH, MIN_HEIGHT, 0, 0);

        let mut whole_view = WholeViewPanel::new_whole_view(wind.width(), wind.height());
        let _whole_layout = whole_view.get_panel();
    
        wind.handle(move |w, e| match e {
            Event::Resize => {
                let (now_width, now_height) = (w.width() as f32, w.height() as f32);
                let mut width_ratio: f32= now_width / DEFAULT_WIDTH as f32;
                let mut height_ratio = now_height / DEFAULT_HEIGHT as f32;
                if width_ratio < 1_f32 {
                    width_ratio = 1f32;
                }
                if height_ratio < 1_f32 {
                    height_ratio = 1_f32;
                }
                // let o = GLOBAL_CHANNEL.0.get_mut();
                // match o {
                //     None => {}
                //     Some(s) => {
                //         (*s).send(NotifyType::Resize(now_width as i32, now_height as i32));
                //     }
                // }
                whole_view.resize_with_ratio(width_ratio, height_ratio);
                true
            }
            _ => false,
        });
    
        wind.end();
        AppWindow{window: wind}
    }

    pub(crate) fn get_window(self) -> Window {
        self.window
    }

}