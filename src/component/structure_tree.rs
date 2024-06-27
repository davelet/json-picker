use std::rc::Rc;
use std::sync::Arc;

use chrono::{Duration, Local};
use fltk::enums::Color;
use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetExt};
use fltk::tree::{Tree, TreeSelect};
use serde_json::Value;
use crate::component::search_bar::SearchBar;

use crate::data::constants::{ACTION_BUTTON_COUNT, ACTION_BUTTON_HEIGHT, SEARCH_BAR_HEIGHT, SEARCH_BTN_WIDTH};
use crate::data::notify_enum::{ComputeStatus, NotifyType};
use crate::data::singleton::{ACTION_BTNS, CHANNEL, TREE_SEARCH_BAR};
use crate::logic::json_handle::{add_tree_items, parse_path_chain};

pub(crate) struct JsonStructure {
    view: Pack,
    tree: Box<Tree>,
}

impl JsonStructure {
    pub(crate) fn new(w: i32, h: i32) -> Self {
        let mut pack = Pack::default().with_size(w, h);
        pack.set_type(PackType::Vertical);
        pack.set_color(Color::Blue);

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
        pack.add(&tree);
        let bar = TREE_SEARCH_BAR.lock().unwrap();
        let mut x = bar.get_bar();
        pack.add(&*x);
        x.hide();

        JsonStructure {
            view: pack,
            tree: Box::new(tree),
        }
    }

    pub(crate) fn view(&self) -> &Pack {
        &self.view
    }

    pub(crate) fn set_tree(&self, json: &Value) {
        let mut tree = self.tree.clone();
        tree.clear();
        add_tree_items(&mut tree, json, String::from("/"));
    }

    pub(crate) fn clear(&self) {
        let root = self.tree.clone().root();
        if let Some(root) = root {
            self.tree.clone().clear_children(&root);
        }
        // self.get_tree().clear() // bug as https://github.com/fltk-rs/fltk-rs/issues/1544
    }

    pub(crate) fn resize(&mut self, width: i32, height: i32) {
        self.view.set_size(width, height);
        let mut guard = TREE_SEARCH_BAR.lock().unwrap();
        let mut margin = 0;
        if guard.get_bar().visible() {
            margin = SEARCH_BAR_HEIGHT;
        }

        self.tree.clone().set_size(width, height - margin);
        guard.resize(width);
    }

}
