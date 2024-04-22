use fltk::{app, prelude::*, window};
use fltk::enums::Color;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 500).with_label("Multi-Row Multi-Column Layout");
    // let mut wind = window::Window::new(100, 100, 400, 400, "Multi-Row Multi-Column Layout");

    // 创建一个 4x4 的网格布局
    let mut grid_pack = Pack::new(10, 10, 38, 38, "grid layout");
    grid_pack.set_type(PackType::Horizontal);
    grid_pack.set_spacing(40);

    for i in 0..4 {
        let mut column_pack = Pack::default().with_size(100, 200).with_label(&*format!("列 {i}"));
        column_pack.set_type(PackType::Vertical);
        column_pack.set_spacing(50);
        column_pack.auto_layout();

        for j in 0..4 {
            let mut frame = Frame::default().with_size(100, 200).with_label(&*format!("{i} j {j}"));
            frame.set_color(Color::from_rgb(
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
            ));
            column_pack.end();
            column_pack.add(&frame);
        }

        grid_pack.end();
        grid_pack.add(&column_pack);
    }

    wind.end();
    wind.make_resizable(true);
    wind.fullscreen(true);
    wind.show();
    wind.set_callback(move |_| {
        grid_pack.redraw();
    });
    app.run().unwrap();
}