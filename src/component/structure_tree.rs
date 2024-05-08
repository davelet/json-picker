use fltk::enums::Color;
use fltk::prelude::WidgetExt;
use fltk::tree::{Tree, TreeItem, TreeSelect};
use serde_json::Value;

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
        for i in 0..5 {
            let ti = TreeItem::new(&tree, &*("item".to_owned() + &*i.to_string()));
            tree.add_item("2/2/3/1", &ti);
        }

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
        match json {
            Value::Bool(_) => {tree.add("Boolean");}
            Value::Number(_) => {tree.add("Number");}
            Value::String(_) => {tree.add("String");}
            Value::Array(_) => {tree.add("Array");}
            Value::Object(map) => {
                for (ele, v) in map {
                    tree.add(&*format!("{ele}: {}", v.is_string()));
                }
            }
            _ => {}
        }
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