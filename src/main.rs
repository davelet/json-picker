use std::any::Any;

use fltk::app::Scheme;
use fltk::enums::{Color, Event};
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::input::MultilineInput;
use fltk::window::DoubleWindow;
use fltk::{app, prelude::*, window};

use crate::component::labeled_box::LabeledBox;
use crate::logic::json_handle;

mod component;
mod data;
mod logic;

const HEADER_HEIGHT: i32 = 20;
const READY: &str = "Ready";

fn main() {
    let mut app = app::App::default();
    app.set_scheme(Scheme::Plastic);
    // let (width, height) = screen_size();
    // println!("screen {} {}", width, height);
    // let width = width as i32;
    // let height = height as i32;
    let (width, height) = (800, 600);

    let mut wind = window::Window::default()
        .with_size(width, height)
        .with_label("Multi-Row Multi-Column Layout");

    let mut center_layout = Pack::new(0, 0, width, height, "");
    center_layout.set_type(PackType::Vertical);

    let line = LabeledBox::new(wind.width(), HEADER_HEIGHT, 3);
    line.child(0).set_label("input area");
    line.child(1).set_label("structure area");
    line.child(2).set_label("view area");
    center_layout.end();
    center_layout.add(&*line.content());

    let double_line_height = HEADER_HEIGHT * 2;
    let mut grid_pack = Pack::new(0, HEADER_HEIGHT, width, height - double_line_height, "");
    grid_pack.set_type(PackType::Horizontal);
    grid_pack.set_spacing(10);

    let mut input = MultilineInput::default().with_size(grid_pack.width() / 3, grid_pack.height());
    grid_pack.end();
    grid_pack.add(&input);

    let mut column_pack = Pack::default()
        .with_size(grid_pack.width() / 3, grid_pack.height())
        .with_label("");
    column_pack.set_type(PackType::Vertical);
    column_pack.set_spacing(10);

    for j in 0..3 {
        let frame = Frame::default()
            .with_size(30, 20)
            .with_label(&*format!("{j} j"));
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

    let foot = LabeledBox::new(wind.width(), HEADER_HEIGHT, 3);
    let mut foot_left = foot.child(0);
    let mut foot_cent = foot.child(1);
    let mut foot_righ = foot.child(2);
    foot_left.set_label(READY);
    foot_cent.set_label("Normal");
    foot_righ.set_label(&*format!("Window: {} x {}", width, height));

    center_layout.end();
    center_layout.add(&*foot.content());

    // callbacks
    input.set_callback(move |inp| {
        foot_left.set_label("Computing");
        let str = serde_json::from_str(&*inp.value());
        match str {
            Ok(json) => {
                result.set_value(&*json_handle::pretty_json(&json));
                foot_cent.set_label("Normal");
            }
            Err(_) => {
                result.set_value("");
                foot_cent.set_label("Illegal input");
            }
        }
        foot_left.set_label(READY);
    });
    wind.handle(move |w, e| match e {
        Event::Resize => {
            resize_content(&mut grid_pack, w.width(), w.height());
            grid_pack.set_size(w.width(), w.height() - double_line_height);
            foot_righ.set_label(&*format!("Window: {} x {}", w.width(), w.height()));
            true
        }
        _ => false,
    });
    let x = wind.handle_event(Event::Resize);
    println!("x = {}", x);

    wind.end();
    wind.make_resizable(true);
    wind.show();
    app.run().unwrap();
}

fn resize_content(content: &mut Pack, width: i32, height: i32) {
    let ii = content.children();
    for i in 0..ii {
        if let Some(mut c) = content.child(i) {
            c.set_size(width / 3, height - HEADER_HEIGHT * 2);
        }
    }
}
