use component::main_panel::GridPanel;
use fltk::app::Scheme;
use fltk::enums::Event;
use fltk::group::{Pack, PackType};
use fltk::{app, prelude::*, window};

use crate::component::labeled_box::LabeledLine;

mod component;
mod data;
mod logic;

const HEADER_HEIGHT: i32 = 20;

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

    let line = LabeledLine::make_header(wind.width(), HEADER_HEIGHT);
    let foot = LabeledLine::init_footer(wind.width(), HEADER_HEIGHT);
    let mut foot_left = foot.child(0);
    let mut foot_cent = foot.child(1);
    let mut foot_righ = foot.child(2);
    foot.display_size(width, height);

    center_layout.end();
    center_layout.add(&*line.content());

    let double_line_height = line.get_height() + foot.get_height();
    let mut grid_pack = GridPanel::new(0, line.get_height(), width, height - double_line_height);

    center_layout.end();
    center_layout.add(&*grid_pack.get_panel());

    center_layout.end();
    center_layout.add(&*foot.content());

    wind.handle(move |w, e| match e {
        Event::Resize => {
            resize_content(&mut grid_pack.get_panel(), w.width(), w.height());
            grid_pack.get_panel().set_size(w.width(), w.height() - double_line_height);
            foot.display_size(w.width(), w.height());
            true
        }
        _ => false,
    });

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
