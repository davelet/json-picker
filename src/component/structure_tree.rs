use fltk::enums::Color;
use fltk::prelude::WidgetExt;
use fltk::tree::{Tree, TreeItem, TreeSelect};
use serde_json::Value;

use crate::logic::json_handle::add_tree_items;

pub(crate) struct JsonStructure {
    view: Box<Tree>,
}

impl JsonStructure {
    pub(crate) fn new(w: i32, h: i32) -> Self {
        let mut tree = Tree::default().with_size(w, h);
        tree.set_root_label(".");
        tree.set_show_root(false);
        tree.set_select_mode(TreeSelect::Multi);
        tree.set_color(Color::Blue);

        tree.set_callback(|t| {
            if let Some(items) = t.get_selected_items() {
                for i in items {
                    println!("{} selected", t.item_pathname(&i).unwrap());
                }
            }
        });

        JsonStructure {
            view: Box::new(tree)
        }
    }

    pub(crate) fn get_tree(&self) -> Box<Tree> {
        self.view.clone()
    }

    pub(crate) fn set_tree(&self, json: &Value) {
        let mut tree = self.get_tree();
        tree.clear();
        add_tree_items(&mut tree, json, String::from("/"));
        
        // tree.set_root_label(".");
    }

    pub(crate) fn clear(&self) {
        // let root = self.get_tree().root();
        // if let Some(root) = root {
        //     self.get_tree().clear_children(&root);
        // }
        self.get_tree().clear()
    }
}