use fltk::{app, prelude::*, window};
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 500).with_label("000000000000000000");
    // let mut wind = window::Window::new(100, 100, 400, 400, "Multi-Row Multi-Column Layout");

    // 创建一个 4x4 的网格布局
    let mut group = Pack::new(10, 10, 380, 380, "grid layout");
    group.set_type(PackType::Horizontal);
    group.set_spacing(10);

    for _ in 0..4 {
        let mut row = Pack::new(0, 0, 380, 95, "2222222222222222222");
        row.set_type(PackType::Vertical);
        row.set_spacing(10);

        for _ in 0..4 {
            let mut frame = Frame::new(0, 0, 95, 95, "33333333333333333333");
            frame.set_color(fltk::enums::Color::from_rgb(
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
            ));
            row.end();
            row.add(&frame);
        }

        group.end();
        group.add(&row);
    }

    wind.end();
    wind.make_resizable(true);
    wind.fullscreen(true);
    wind.show();
    app.run().unwrap();
}