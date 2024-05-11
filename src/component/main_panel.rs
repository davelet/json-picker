use crate::component::structure_tree::JsonStructure;
use crate::data::notify_enum::{ComputeResult, ComputeStatus, NotifyType};
use fltk::enums::Event;
use fltk::tree::Tree;
use fltk::{
    enums::Color,
    group::{Pack, PackType},
    input::MultilineInput,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
};
use fltk::prelude::DisplayExt;
use fltk::text::{TextBuffer, TextEditor};

use crate::data::constants::{CHANNEL, COLUMN_COUNT, JSON_SIZE_LIMIT, JSON_SIZE_WARN};
use crate::logic::json_handle;

pub(crate) struct ContentPanel {
    panel: Box<Pack>,
    left: Box<TextEditor>,
    center: Box<Tree>,
    right: Box<MultilineInput>,
}

impl ContentPanel {
    pub(crate) fn new_content_view(width: i32, height: i32) -> Self {
        let mut grid_pack = Pack::default().with_size(width, height);
        grid_pack.set_type(PackType::Horizontal);
        // grid_pack.set_spacing(10);

        let mut buf = TextBuffer::default();

        let mut input = TextEditor::default().with_size(width / COLUMN_COUNT, height);
        input.set_buffer(buf.clone());
        grid_pack.end();
        grid_pack.add(&input);

        let tree_view = JsonStructure::new(width / COLUMN_COUNT, height);
        let tree = *tree_view.get_tree();

        grid_pack.end();
        grid_pack.add(&tree);

        let mut result = MultilineInput::default().with_size(width / 3, height);
        result.set_readonly(true);
        result.set_color(Color::Gray0);
        grid_pack.end();
        grid_pack.add(&result);

        let right = Box::new(result);
        let mut right_box = right.clone();
        input.handle(move |i, e| match e {
            Event::Unfocus => {
                let text = &*buf.text();
                if text.trim().len() == 0 {
                    return true;
                }
                let s = CHANNEL.0.clone();
                if text.len() > JSON_SIZE_LIMIT { // move to `settings`
                    s.send(NotifyType::Result(ComputeResult::Error(JSON_SIZE_WARN.to_string())));
                    return true;
                }
                s.send(NotifyType::Status(ComputeStatus::Computing));
                let str = serde_json::from_str(text);
                match str {
                    Ok(json) => {
                        tree_view.set_tree(&json);
                        right_box.set_value(&*json_handle::pretty_json(&json));
                        s.send(NotifyType::Result(ComputeResult::Normal));
                    }
                    Err(er) => {
                        tree_view.clear();
                        right_box.set_value("");
                        s.send(NotifyType::Result(ComputeResult::Error(er.to_string())));
                    }
                }
                s.send(NotifyType::Status(ComputeStatus::Ready));
                true
            }
            _ => false,
        });

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
