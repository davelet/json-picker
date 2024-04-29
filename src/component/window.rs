use fltk::{enums::Event, prelude::{GroupExt, WidgetBase, WidgetExt}, window::{self, Window}};

use crate::data::{DEFAULT_HEIIGHT, DEFAULT_WIDTH, MIN_HEIIGHT, MIN_WIDTH};

use super::border_panel::WholeViewPanel;

pub(crate) struct AppWindow {
    window: Window
}

impl AppWindow {
    pub(crate) fn new() -> Self{
        let (width, height) = (DEFAULT_WIDTH, DEFAULT_HEIIGHT);

        let mut wind = window::Window::default()
            .with_size(width, height)
            .with_label("JSON HAND");
    
        let mut whole_view = WholeViewPanel::new_whole_view(0, 0, wind.width(), wind.height());
        let whole_layout = whole_view.get_panel();
    
        // let (s, r) = app::channel();
        wind.handle(move |w, e| match e {
            Event::Resize => {
                // whole_view.on_parent_resize(w.width(), w.height());
                let (mut now_width, mut now_height) = (w.width() as f32, w.height() as f32);
                let mut too_small = false;
                if now_width < MIN_WIDTH as f32 {
                    too_small = true;
                    now_width = MIN_WIDTH as f32;
                }
                if now_height < MIN_HEIIGHT as f32 {
                    too_small = true;
                    now_height = MIN_HEIIGHT as f32;
                }
                if too_small {
                    w.set_size(now_width as i32, now_height as i32);
                    return false;
                }
                let mut width_ratio: f32= now_width / DEFAULT_WIDTH as f32;
                let mut height_ratio = now_height / DEFAULT_HEIIGHT as f32;
                if width_ratio < 1_f32 {
                    width_ratio = 1f32;
                }
                if height_ratio < 1_f32 {
                    height_ratio = 1_f32;
                }
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