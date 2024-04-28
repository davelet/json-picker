use fltk::{enums::Event, prelude::{GroupExt, WidgetBase, WidgetExt}, window::{self, Window}};
use crate::component::feature::CustomizedAction;

use super::main_panel::ContentPanel;

pub(crate) struct AppWindow {
    window: Window
}

impl AppWindow {
    pub(crate) fn new() -> Self{
        let (width, height) = (800, 600);

        let mut wind = window::Window::default()
            .with_size(width, height)
            .with_label("JSON HAND");
    
        let whole_view = ContentPanel::new_whole_view(0, 0, wind.width(), wind.height());
        let whole_layout = whole_view.get_panel();
    
        // let (s, r) = app::channel();
        wind.handle(move |w, e| match e {
            Event::Resize => {
                whole_view.on_parent_resize(w.width(), w.height());
        //         // resize_content(&mut grid_pack.get_panel(), w.width(), w.height());
        //         // grid_pack
        //         //     .get_panel()
        //         //     .set_size(w.width(), w.height() - double_line_height);
        //         // foot.display_size(w.width(), w.height());
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

    // fn resize_content(content: &mut Pack, width: i32, height: i32) {
    //     let ii = content.children();
    //     for i in 0..ii {
    //         if let Some(mut c) = content.child(i) {
    //             c.set_size(width / 3, height - HEADER_HEIGHT * 2);
    //         }
    //     }
    // }
}