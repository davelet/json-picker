use crate::component::structure_tree::JsonStructure;
use crate::data::notify_enum::{ComputeResult, ComputeStatus, NotifyType};
use fltk::enums::Event;
use fltk::tree::Tree;
use fltk::{app, enums::Color, group::{Pack, PackType}, prelude::{GroupExt, WidgetBase, WidgetExt}};
use fltk::prelude::DisplayExt;
use fltk::text::{TextBuffer, TextDisplay, TextEditor};
use serde_json::Value;

use crate::data::constants::{COLUMN_COUNT, JSON_SIZE_LIMIT, JSON_SIZE_WARN};
use crate::data::singleton::{CHANNEL, GLOBAL_JSON, JSON_INPUT_BOX, RESUTL_CONTROL, TREE_VIEW};
use crate::logic::json_handle;

pub(crate) struct ContentPanel {
    panel: Box<Pack>,
    left: Box<TextEditor>,
    center: Box<Tree>,
    right: Box<TextDisplay>,
}

impl ContentPanel {
    pub(crate) fn new(width: i32, height: i32) -> Self {
        let mut grid_pack = Pack::default().with_size(width, height);
        grid_pack.set_type(PackType::Horizontal);
        // grid_pack.set_spacing(10);

        let input = JSON_INPUT_BOX.lock().unwrap();
        let mut input = (*input).clone();
        input.set_buffer(TextBuffer::default());
        grid_pack.end();
        grid_pack.add(&input);

        let tree_view = TREE_VIEW.lock().unwrap();
        let tree = *tree_view.get_tree();

        grid_pack.end();
        grid_pack.add(&tree);

        let mut result = TextDisplay::default().with_size(width / 3, height);
        result.set_buffer(RESUTL_CONTROL.lock().unwrap().clone());
        result.set_text_color(Color::Blue);
        grid_pack.end();
        grid_pack.add(&result);

        let right = Box::new(result);
        // let right_box = right.clone();


        ContentPanel {
            panel: Box::new(grid_pack),
            left: Box::new(input),
            center: Box::new(tree),
            right,
        }
    }

    pub(crate) fn get_panel(&self) -> Box<Pack> {
        self.panel.clone()
    }

    pub(crate) fn resize_with_parent_size(&mut self, parent_w: i32, parent_h: i32) {
        let mut pack = *self.get_panel();
        pack.set_size(parent_w, parent_h);
        self.left.set_size(parent_w / COLUMN_COUNT, parent_h);
        self.center.set_size(parent_w / COLUMN_COUNT, parent_h);
        self.right.set_size(parent_w / COLUMN_COUNT, parent_h);
    }
}
