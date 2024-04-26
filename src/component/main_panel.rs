use fltk::{
    enums::Color,
    frame::Frame,
    group::{Pack, PackType},
    input::MultilineInput,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
};

use crate::logic::json_handle;

use super::labeled_box::LabeledLine;

// grid panel, a vetical container, includes header, content, footer.
// pub(crate) struct GridPanel {

// }

/// content panel, a horizontal container, in which there are 3 columns: input part, tree view, and result view
pub(crate) struct ContentPanel {
    panel: Box<Pack>,
}

impl ContentPanel {
    pub(crate) fn new_whole_view(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut whole_view = Pack::new(0, 0, width, height, "");
        whole_view.set_type(PackType::Vertical);
    
        let line = LabeledLine::make_header(width);
        let foot = LabeledLine::init_footer(width);
        foot.display_size(width, height);
    
        whole_view.end();
        whole_view.add(&*line.content());
    
        let double_line_height = line.get_height() + foot.get_height();
        let grid_pack = ContentPanel::new_content(0, line.get_height(), width, height - double_line_height);
    
        whole_view.end();
        whole_view.add(&*grid_pack.get_panel());
    
        whole_view.end();
        whole_view.add(&*foot.content());
        ContentPanel {
            panel: Box::new(whole_view)
        }
    }

    pub(crate) fn new_content(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut grid_pack = Pack::new(x, y, width, height, "");
        grid_pack.set_type(PackType::Horizontal);
        // grid_pack.set_spacing(10);

        let mut input = MultilineInput::default().with_size(width / 3, height);
        grid_pack.end();
        grid_pack.add(&input);

        let mut column_pack = Pack::default().with_size(width / 3, height).with_label("");
        column_pack.set_type(PackType::Vertical);
        column_pack.set_spacing(10);

        for j in 0..3 {
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

        input.set_callback(move |inp| {
            // foot_left.set_label("Computing");
            let str = serde_json::from_str(&*inp.value());
            match str {
                Ok(json) => {
                    result.set_value(&*json_handle::pretty_json(&json));
                    // foot_cent.set_label("Normal");
                }
                Err(_) => {
                    result.set_value("");
                    // foot_cent.set_label("Illegal input");
                }
            }
            // foot_left.set_label(READY);
        });

        ContentPanel {
            panel: Box::new(grid_pack)
        }
    }

    pub(crate) fn get_panel(&self) -> Box<Pack> {
        self.panel.clone()
    }
}
