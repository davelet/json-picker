#[cfg(not(debug_assertions))]
use std::env;

use fltk::{app, dialog, menu, prelude::{GroupExt, WidgetExt}, window::Window};
use fltk::enums::{Color, Event, FrameType, Shortcut};
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::image::PngImage;
use fltk::input::IntInput;
use fltk::menu::SysMenuBar;
use fltk::prelude::{ImageExt, InputExt, MenuExt, WidgetBase, WindowExt};

use crate::data::constants::{APP_NAME, DEFAULT_HEIGHT, DEFAULT_WIDTH, MIN_HEIGHT, MIN_WIDTH, SYS_SETTINGS_INPUT_HEIGHT, SYS_SETTINGS_LINE_HEIGHT, SYS_SETTINGS_WINDOW_WIDTH};
use crate::data::singleton::{APP_WINDOW, WHOLE_VIEW};
use crate::logic::app_settings::{get_limit, set_limit};
use crate::logic::app_startup;

pub(crate) struct AppWindow {
    window: Window,
}

impl AppWindow {
    pub(crate) fn new() -> Self {
        let mut wind = Window::default()
            .with_size(DEFAULT_WIDTH, DEFAULT_HEIGHT)
            .with_label(APP_NAME);
        wind.size_range(MIN_WIDTH, MIN_HEIGHT, 0, 0);

        let mut m = SysMenuBar::default();
        m.clear();
        init_menu(&mut m);

        let whole_view = WHOLE_VIEW.lock().unwrap();
        wind.add(&*whole_view.get_panel());
        wind.end();
        AppWindow { window: wind }
    }

    pub(crate) fn get_window(&mut self) -> &mut Window {
        &mut self.window
    }
}

fn init_menu(m: &mut SysMenuBar) {
    let reset_menu = "&App/Reset";
    m.add(
        reset_menu,
        Shortcut::None,
        menu::MenuFlag::Normal,
        |a| {
            let (mut x, mut y) = (0, 0);
            {
                let w = &APP_WINDOW.lock().unwrap().window;
                let height = w.h();
                x = w.x() + 50;
                y = w.y() + height / 2 - 30;
            }
            let txt = "Attention: This will let the app be out of box and shut it down. You have to restart it!";
            let r = dialog::choice2(x, y, txt, "More...", "Cancel", "Continue");
            if let Some(2) = r {
                app_startup::clear();
                app::quit();
            } else if let Some(0) = r {
                let msg = r#"Reset action will clear all cached data, including position, window size, content snapshot,
                and your customized config data, like json limit.
                After your restart, the app would look like you just download it."#;
                dialog::message_default(msg);
                a.find_item(reset_menu).unwrap().do_callback(a);
            }
        },
    );
    m.add(
        "&App/Config",
        Shortcut::Meta | ',',
        menu::MenuFlag::Normal,
        |_| {
            let mut sw = SettingWindow::new();
            sw.window.show();
        },
    );
}

struct SettingWindow {
    window: Window,
}

impl SettingWindow {
    fn new() -> Self {
        let h = SYS_SETTINGS_INPUT_HEIGHT + SYS_SETTINGS_LINE_HEIGHT * 2;
        let mut window = Window::default().with_size(SYS_SETTINGS_WINDOW_WIDTH, h).with_label("Settings");
        window.make_modal(true);

        let mut pack = Pack::default().size_of_parent();
        pack.set_type(PackType::Vertical);

        // let mut lb = LightButton::default().with_size(60, 30).with_label("bt1n");
        // lb.set_color(Color::Green);
        // lb.set_label_color(Color::Blue);
        //
        // Button::default().with_size(60, 30).with_label("btn2");

        Frame::default().with_size(pack.w(), SYS_SETTINGS_LINE_HEIGHT).with_label("Please set input length limit for json input:");
        let mut limit = IntInput::default().with_size(pack.w(), SYS_SETTINGS_INPUT_HEIGHT);
        limit.set_value(&*get_limit().to_string());
        Frame::default().with_size(pack.w(), SYS_SETTINGS_LINE_HEIGHT).with_label("Close the settings window to update limit value.").set_label_color(Color::Blue);

        pack.end();

        window.end();

        window.handle(move |w, e| {
            if let Event::Hide = e {
                let l = limit.value();
                let lv: i64 = l.parse().unwrap();
                if lv > 0 {
                    set_limit(lv);
                }
                true
            } else {
                false
            }
        });
        Self { window }
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
        #[cfg(not(debug_assertions))] // for release
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
        let window = &mut self.window;
        let w = window.w();
        let h = window.h();
        let x = x as i32 / 2 - w / 2;
        let y = y as i32 / 2 - h / 2;
        window.resize(x, y, w, h);
    }
}
