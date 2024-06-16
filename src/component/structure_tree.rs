use chrono::{Duration, Local};
use fltk::enums::Color;
use fltk::prelude::WidgetExt;
use fltk::tree::{Tree, TreeSelect};
use serde_json::Value;

use crate::data::notify_enum::{ComputeStatus, NotifyType};
use crate::data::singleton::CHANNEL;
use crate::logic::json_handle::{add_tree_items, parse_path_chain};

pub(crate) struct JsonStructure {
    view: Box<Tree>,
}

impl JsonStructure {
    pub(crate) fn new(w: i32, h: i32) -> Self {
        let mut tree = Tree::default().with_size(w, h);
        tree.set_show_root(false);
        tree.set_select_mode(TreeSelect::Single);
        tree.set_color(Color::Blue);

        tree.set_callback(|t| {
            if let Some(items) = t.get_selected_items() {
                let mut paths = vec![];
                for i in items {
                    let chain = parse_path_chain(&i);
                    paths.push(chain);
                }
                let now = Local::now();
                let two_sec_later = now + Duration::seconds(2);
                CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Waiting(two_sec_later, paths)));
            }
        });

        JsonStructure {
            view: Box::new(tree),
        }
    }

    pub(crate) fn get_tree(&self) -> Box<Tree> {
        self.view.clone()
    }

    pub(crate) fn set_tree(&self, json: &Value) {
        let mut tree = self.get_tree();
        tree.clear();
        add_tree_items(&mut tree, json, String::from("/"));
    }

    pub(crate) fn clear(&self) {
        let root = self.get_tree().root();
        if let Some(root) = root {
            self.get_tree().clear_children(&root);
        }
        // self.get_tree().clear() // bug as https://github.com/fltk-rs/fltk-rs/issues/1544
    }

}
