use std::env;
use fltk::{prelude::{GroupExt, WidgetExt}, window::Window};
use fltk::enums::FrameType;
use fltk::frame::Frame;
use fltk::image::{Image, PngImage};
use fltk::prelude::{ImageExt, WindowExt};

use crate::data::constants::{APP_NAME, DEFAULT_HEIGHT, DEFAULT_WIDTH, MIN_HEIGHT, MIN_WIDTH};
use crate::data::singleton::WHOLE_VIEW;

pub(crate) struct AppWindow {
    window: Window,
}

impl AppWindow {
    pub(crate) fn new() -> Self {
        let mut wind = Window::default()
            .with_size(DEFAULT_WIDTH, DEFAULT_HEIGHT)
            .with_label(APP_NAME);
        wind.size_range(MIN_WIDTH, MIN_HEIGHT, 0, 0);

        let whole_view = WHOLE_VIEW.lock().unwrap();
        wind.add(&*whole_view.get_panel());
        wind.end();
        AppWindow { window: wind }
    }

    pub(crate) fn get_window(&mut self) -> &mut Window {
        &mut self.window
    }
}

pub(crate) struct StartupWindow {
    window: Window,
}

impl StartupWindow {
    pub(crate) fn new(width: i32, height: i32) -> Self {
        let mut window = Window::default().with_size(width, height);

        let mut title_frame = Frame::default().with_size(width, (height as f64 * 0.2) as i32).with_label(APP_NAME);
        title_frame.set_label_size(30);

        let scale = 0.7;
        let mut frame = Frame::default().with_size((width as f64 * scale) as i32, (height as f64 * scale) as i32).center_of(&window);
        frame.set_frame(FrameType::EngravedBox);
        let mut png = None;
        #[cfg(debug_assertions)]
        if let Ok(mut icon) = PngImage::load("assets/icon.png") {
            icon.scale(200, 200, true, true);
            png = Some(icon);
        }
        #[cfg(not(debug_assertions))] //after release
        if let Ok(exe) = env::current_exe() {
            let resources_path = exe.parent().expect("Failed to get parent directory")
                .parent().expect("Failed to get parent directory2")
                .join("Resources")
                .join("assets")
                .join("icon.png");
            let app_png = PngImage::load(resources_path);
            if let Ok(mut icon) = app_png {
                icon.scale(200, 200, true, true);
                png = Some(icon);
            }
        }

        frame.set_image(png);
        window.end();
        window.set_border(false);

        StartupWindow { window }
    }

    pub(crate) fn get(&mut self) -> &mut Window {
        &mut self.window
    }

    pub(crate) fn pin(&mut self, (x, y): (f64, f64)) {
        let mut window = &mut self.window;
        let w = window.w();
        let h = window.h();
        let x = x as i32 / 2 - w / 2;
        let y = y as i32 / 2 - h / 2;
        window.resize(x, y, w, h);
    }
}
