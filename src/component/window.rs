use fltk::{enums::Event, prelude::{GroupExt, WidgetBase, WidgetExt}, window::{self, Window}};
use fltk::prelude::WindowExt;

use crate::data::{APP_NAME, DEFAULT_HEIGHT, DEFAULT_WIDTH, MIN_HEIGHT, MIN_WIDTH};

use super::border_panel::WholeViewPanel;

pub(crate) struct AppWindow {
    window: Window,
}

impl AppWindow {
    pub(crate) fn new() -> Self{
        let (width, height) = (DEFAULT_WIDTH, DEFAULT_HEIGHT);

        let mut wind = window::Window::default()
            .with_size(width, height)
            .with_label(APP_NAME);
        wind.size_range(MIN_WIDTH, MIN_HEIGHT, 0, 0);

        let mut whole_view = WholeViewPanel::new_whole_view(wind.width(), wind.height());
        let _whole_layout = whole_view.get_panel();

        wind.handle(move |_, e| match e {
            Event::Resize => {
                whole_view.resize_with_auto_detect_size();
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
