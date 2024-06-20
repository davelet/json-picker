use fltk::{enums::Color, group::{Pack, PackType}, prelude::{GroupExt, WidgetBase, WidgetExt}};
use fltk::prelude::DisplayExt;
use fltk::text::{TextBuffer, TextDisplay, TextEditor};
use fltk::tree::Tree;

use crate::data::constants::{COLUMN_COUNT, SEARCH_BAR_HEIGHT};
use crate::data::singleton::{JSON_INPUT_BOX, RESUTL_VIEW, TREE_VIEW};

pub(crate) struct ContentPanel {
    panel: Box<Pack>,
    left: Box<TextEditor>,
    // center: Box<Pack>,
    right: Box<TextDisplay>,
}

impl ContentPanel {
    pub(crate) fn new(width: i32, height: i32) -> Self {
        let mut grid_pack = Pack::default().with_size(width, height);
        grid_pack.set_type(PackType::Horizontal);

        let input = JSON_INPUT_BOX.lock().unwrap();
        let mut input = (*input).clone();
        input.set_buffer(TextBuffer::default());
        grid_pack.add(&input);

        let tree_view = TREE_VIEW.lock().unwrap();
        let tree_pack = tree_view.view();
        grid_pack.add(tree_pack);

        let mut result = TextDisplay::default().with_size(width / 3, height);
        result.set_buffer(RESUTL_VIEW.lock().unwrap().clone());
        result.set_text_color(Color::Blue);
        grid_pack.add(&result);
        grid_pack.end();

        let right = Box::new(result);
        ContentPanel {
            panel: Box::new(grid_pack),
            left: Box::new(input),
            // center: Box::new(tree),
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
        // self.center.set_size(parent_w / COLUMN_COUNT, parent_h);
        self.right.set_size(parent_w / COLUMN_COUNT, parent_h);
    }
}
