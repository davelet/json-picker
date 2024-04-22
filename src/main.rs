use fltk::{app, prelude::*, window};
use fltk::app::screen_size;
use fltk::enums::Color;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};

fn main() {
    let app = app::App::default();
    let (width, height) = screen_size();
    println!("screen {} {}", width, height);
    let width = width as i32;
    let height = height as i32;
    let mut wind = window::Window::default().with_size(width, height).with_label("Multi-Row Multi-Column Layout");

    let col_count = 4;
    let row_count = 3;
    // 创建一个 4x4 的网格布局
    let mut grid_pack = Pack::new(0, 0, width, height, "grid layout");
    grid_pack.set_type(PackType::Horizontal);
    grid_pack.set_spacing(10);

    for i in 0..col_count {
        let mut column_pack = Pack::default().with_size(wind.width() / col_count, wind.height()).with_label(&*format!("列 {i}"));
        column_pack.set_type(PackType::Vertical);
        column_pack.set_spacing(10);
        // column_pack.auto_layout();
        column_pack.set_color(Color::from_rgb(
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
        ));


        for j in 0..row_count {
            let mut frame = Frame::default().with_size(column_pack.width(), column_pack.height() / row_count).with_label(&*format!("{i} j {j}"));
            column_pack.end();
            column_pack.add(&frame);

            column_pack.set_callback(move |_| frame.redraw())
        }

        grid_pack.end();
        grid_pack.add(&column_pack);

        grid_pack.set_callback(move |_| {
            column_pack.redraw();
        })
    }

    wind.end();
    wind.make_resizable(true);
    // wind.fullscreen(true);
    wind.show();
    wind.set_callback(move |_| {
        grid_pack.redraw();
    });
    app.run().unwrap();
}