use fltk::{app, prelude::*, window};
use fltk::app::{Scheme, screen_size};
use fltk::enums::Color;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::input::MultilineInput;

const HEADER_HEIGHT: i32 = 20;

fn main() {
    let mut app = app::App::default();
    app.set_scheme(Scheme::Plastic);
    let (width, height) = screen_size();
    println!("screen {} {}", width, height);
    let width = width as i32;
    let height = height as i32;
    let mut wind = window::Window::default().with_size(width, height).with_label("Multi-Row Multi-Column Layout");

    let mut head_line = Pack::default().with_size(wind.width(), HEADER_HEIGHT);
    head_line.set_type(PackType::Horizontal);
    let input_head = Frame::default().with_size(wind.width() / 3, HEADER_HEIGHT).with_label("input area");
    let structure_head = Frame::default().with_size(wind.width() / 3, HEADER_HEIGHT).with_label("tree area");
    let view_head = Frame::default().with_size(wind.width() / 3, HEADER_HEIGHT).with_label("result area");
    head_line.end();

    let mut grid_pack = Pack::new(0, HEADER_HEIGHT, width, height, "");
    grid_pack.set_type(PackType::Horizontal);
    grid_pack.set_spacing(10);

    let mut input = MultilineInput::default().with_size(wind.width() / 3, wind.height() - HEADER_HEIGHT);
    grid_pack.end();
    grid_pack.add(&input);

    let mut column_pack = Pack::default().with_size(wind.width() / 3, wind.height()).with_label("");
    column_pack.set_type(PackType::Vertical);
    column_pack.set_spacing(10);
    // column_pack.auto_layout();
    column_pack.set_color(Color::from_rgb(
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>(),
    ));

    for j in 0..3 {
        let mut frame = Frame::default().with_size(column_pack.width(), column_pack.height() / 3).with_label(&*format!("{j} j"));
        column_pack.end();
        column_pack.add(&frame);

        // column_pack.set_callback(move |_| frame.redraw())
    }

    grid_pack.end();
    grid_pack.add(&column_pack);

    // grid_pack.set_callback(move |_| {
    // column_pack.redraw();
    // })
    let mut result = MultilineInput::default().with_size(wind.width() / 3, wind.height() - HEADER_HEIGHT);
    result.set_readonly(true);
    result.set_color(Color::Gray0);
    grid_pack.end();
    grid_pack.add(&result);

    /// callbacks
    input.set_callback(move |inp| {
        println!("moved {}", inp.value());
        result.set_value(&*inp.value());
    });

    wind.end();
    wind.make_resizable(true);
    // wind.fullscreen(true);
    wind.show();
    // wind.set_callback(move |_| {
    //     grid_pack.redraw();
    // });
    app.run().unwrap();
}