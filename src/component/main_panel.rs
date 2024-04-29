use fltk::{
    enums::Color,
    frame::Frame,
    group::{Pack, PackType},
    input::MultilineInput,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
};

use crate::data::COLUMN_COUNT;

pub(crate) struct ContentPanel {
    panel: Box<Pack>,
    left: Box<MultilineInput>,
    center: Box<Pack>,
    right: Box<MultilineInput>,
}

impl ContentPanel {
    pub(crate) fn new_content_view(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut grid_pack = Pack::new(x, y, width, height, "");
        grid_pack.set_type(PackType::Horizontal);
        // grid_pack.set_spacing(10);

        let mut input = MultilineInput::default().with_size(width / COLUMN_COUNT, height);
        grid_pack.end();
        grid_pack.add(&input);

        let mut column_pack = Pack::default().with_size(width / COLUMN_COUNT, height).with_label("");
        column_pack.set_type(PackType::Vertical);
        column_pack.set_spacing(10);

        for j in 0..COLUMN_COUNT {
            let frame = Frame::default()
                .with_size(30, 20)
                .with_label(&*format!("{j} j"));
            column_pack.end();
            column_pack.add(&frame);
        }

        grid_pack.end();
        grid_pack.add(&column_pack);

        let mut result = MultilineInput::default().with_size(width / 3, height);
        result.set_readonly(true);
        result.set_color(Color::Gray0);
        grid_pack.end();
        grid_pack.add(&result);

        // input.set_callback(move |inp| {
        //     // foot_left.set_label("Computing");
        //     let str = serde_json::from_str(&*inp.value());
        //     match str {
        //         Ok(json) => {
        //             result.set_value(&*json_handle::pretty_json(&json));
        //             // foot_cent.set_label("Normal");
        //         }
        //         Err(_) => {
        //             result.set_value("");
        //             // foot_cent.set_label("Illegal input");
        //         }
        //     }
        //     // foot_left.set_label(READY);
        // });

        ContentPanel {
            panel: Box::new(grid_pack),
            left: Box::new(input),
            center: Box::new(column_pack),
            right: Box::new(result),
        }
    }

    pub(crate) fn get_panel(&self) -> Box<Pack> {
        self.panel.clone()
    }

    pub(crate) fn resize_with_ratio(&mut self, parent_w: i32, parent_h: i32, ratio_w: f32, ratio_h: f32) {
        
    }
}

