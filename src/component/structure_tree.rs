use fltk::enums::Color;
use fltk::prelude::WidgetExt;
use fltk::tree::{Tree, TreeSelect};
use regex::Regex;
use serde_json::Value;

use crate::data::constants::CHANNEL;
use crate::logic::json_handle::{add_tree_items, parse_path_chain};

pub(crate) struct JsonStructure {
    view: Box<Tree>,
    selected_path: Vec<String>,
}

impl JsonStructure {
    pub(crate) fn new(w: i32, h: i32) -> Self {
        let mut tree = Tree::default().with_size(w, h);
        // tree.set_root_label(".");
        tree.set_show_root(false);
        tree.set_select_mode(TreeSelect::Multi);
        tree.set_color(Color::Blue);

        tree.set_callback(|t| {
            if let Some(items) = t.get_selected_items() {
                for i in items {
                    let mut chain = parse_path_chain(&i);
                      println!("get the resutl");
                      loop {
                          let s = chain.pop();
                          match s {
                              None => {break}
                              Some(s1) => {println!("chain = {s1}")}
                          }
                      }
                    // if let Ok(p) = t.item_pathname(&i) {
                    //     println!("{} selected", p);
                    //     let re = Regex::new(r"/").unwrap();
                    //     if re.is_match(&*p) {
                    //         println!("The string contains a slash.");
                    //     }
                    // }
                }
                // CHANNEL.0.clone().send(val);
            }
        });

        JsonStructure {
            view: Box::new(tree),
            selected_path: vec![],
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
        let root = self.get_tree().root();
        if let Some(root) = root {
            self.get_tree().clear_children(&root);
        }
        // self.get_tree().clear() // bug as https://github.com/fltk-rs/fltk-rs/issues/1544
    }

    pub(crate) fn get_selected_path(&self) -> &Vec<String> {
        return &self.selected_path;
    }
}
