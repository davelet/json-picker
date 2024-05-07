use fltk::enums::Color;
use fltk::prelude::WidgetExt;
use fltk::tree::{Tree, TreeItem, TreeSelect};
use serde_json::Value;

pub(crate) struct JsonStructure {
    view: Box<Tree>
}

impl JsonStructure {
    pub(crate) fn new(w: i32, h: i32) -> Self {
        let mut tree = Tree::default().with_size(w, h);
        tree.set_root_label(".");
        tree.set_select_mode(TreeSelect::Multi);
        tree.set_color(Color::Blue);
        for i in 0 .. 5 {
            let ti = TreeItem::new(&tree, &*("item".to_owned() + &*i.to_string()));
            tree.add_item("2/2/3/1", &ti);
        }

        // for i in 0 .. 5 {
        //     tree.add(&*("item".to_owned() + &*i.to_string()));
        // }
        // tree.add("2");
        // tree.add("1/2");
        // tree.add("1/3");
        // tree.add("1/4");
        // tree.add("1/4/3");
        // tree.add("1/4/4");

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
        
    }

    pub(crate) fn clear(&self) {
        self.get_tree().clear();
    }
}