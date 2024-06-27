use std::rc::Rc;
use std::sync::Arc;

use chrono::{Duration, Local};
use fltk::enums::Color;
use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetExt};
use fltk::tree::{Tree, TreeSelect};
use serde_json::Value;
use crate::component::search_bar::SearchBar;

use crate::data::constants::{ACTION_BUTTON_COUNT, ACTION_BUTTON_HEIGHT, SEARCH_BAR_HEIGHT};
use crate::data::notify_enum::{ComputeStatus, NotifyType};
use crate::data::singleton::{ACTION_BTNS, CHANNEL};
use crate::logic::json_handle::{add_tree_items, parse_path_chain};

pub(crate) struct JsonStructure {
    view: Pack,
    tree: Box<Tree>,
    search_bar: Box<SearchBar>,
}

impl JsonStructure {
    pub(crate) fn new(w: i32, h: i32) -> Self {
        let mut pack = Pack::default().with_size(w, h);
        pack.set_type(PackType::Vertical);
        pack.set_color(Color::Blue);

        let mut tree = Tree::default().with_size(w, h - SEARCH_BAR_HEIGHT);
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
        let bar = SearchBar::new(w);
        // action_bar.hide();
        pack.add(&*bar.get_bar());

        JsonStructure {
            view: pack,
            tree: Box::new(tree),
            search_bar: Box::new(bar),
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
        self.tree.clone().set_size(width, height - SEARCH_BAR_HEIGHT);
        self.search_bar.resize(width);
    }

}
