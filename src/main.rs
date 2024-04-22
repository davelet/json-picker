use std::any::Any;

use fltk::{app, prelude::*, window};
use fltk::app::{Scheme, screen_size};
use fltk::enums::Color;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::input::MultilineInput;
use fltk::window::DoubleWindow;
use crate::json_handle::pretty_json;

mod json_handle;

const HEADER_HEIGHT: i32 = 20;
const READY: &str = "Ready";

fn main() {
    let mut app = app::App::default();
    app.set_scheme(Scheme::Plastic);
    let (width, height) = screen_size();
    println!("screen {} {}", width, height);
    let width = width as i32;
    let height = height as i32;
    let mut wind = window::Window::default().with_size(width, height).with_label("Multi-Row Multi-Column Layout");

    let mut center_layout = Pack::new(0, 0, width, height, "");
    center_layout.set_type(PackType::Vertical);

    let (line, mut input_area, mut structure, mut view_area) = build_lane(&mut wind);
    input_area.set_label("input area");
    structure.set_label("structure area");
    view_area.set_label("view area");
    center_layout.end();
    center_layout.add(&line);

    let mut grid_pack = Pack::new(0, HEADER_HEIGHT, width, height - HEADER_HEIGHT * 2, "");
    grid_pack.set_type(PackType::Horizontal);
    grid_pack.set_spacing(10);

    let mut input = MultilineInput::default().with_size(grid_pack.width() / 3, grid_pack.height());
    grid_pack.end();
    grid_pack.add(&input);

    let mut column_pack = Pack::default().with_size(grid_pack.width() / 3, grid_pack.height()).with_label("");
    column_pack.set_type(PackType::Vertical);
    column_pack.set_spacing(10);

    for j in 0..3 {
        let mut frame = Frame::default().with_size(30, 20).with_label(&*format!("{j} j"));
        column_pack.end();
        column_pack.add(&frame);

        // column_pack.set_callback(move |_| frame.redraw())
    }

    grid_pack.end();
    grid_pack.add(&column_pack);

    // grid_pack.set_callback(move |_| {
    // column_pack.redraw();
    // })
    let mut result = MultilineInput::default().with_size(grid_pack.width() / 3, grid_pack.height());
    result.set_readonly(true);
    result.set_color(Color::Gray0);
    grid_pack.end();
    grid_pack.add(&result);

    center_layout.end();
    center_layout.add(&grid_pack);

    let mut foot = build_lane(&mut wind);
    foot.1.set_label(READY);
    foot.2.set_label("Normal");
    foot.3.set_label(&*format!("Screen: {} x {}", width, height));

    center_layout.end();
    center_layout.add(&foot.0);

    /// callbacks
    input.set_callback(move |inp| {
        foot.1.set_label("Computing");
        let str = serde_json::from_str(&*inp.value());
        match str {
            Ok(json) => {
                result.set_value(&*pretty_json(&json));
                foot.2.set_label("Normal");
            }
            Err(_) => {
                result.set_value("");
                foot.2.set_label("Illegal input");
            }
        }
        foot.1.set_label(READY);
    });

    wind.end();
    wind.make_resizable(true);
    wind.fullscreen(true);
    wind.show();
    // wind.set_callback(move |_| {
    //     grid_pack.redraw();
    // });
    app.run().unwrap();
}

fn build_lane(mut wind: &mut DoubleWindow) -> (Pack, Frame, Frame, Frame){
    let mut lane = Pack::default().with_size(wind.width(), HEADER_HEIGHT);
    lane.set_type(PackType::Horizontal);
    let left = Frame::default().with_size(wind.width() / 3, HEADER_HEIGHT);
    let center = Frame::default().with_size(wind.width() / 3, HEADER_HEIGHT);
    let right = Frame::default().with_size(wind.width() / 3, HEADER_HEIGHT);
    lane.end();
    (lane, left, center, right)
}