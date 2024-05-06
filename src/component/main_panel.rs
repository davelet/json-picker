use crate::component::structure_tree::JsonStructure;
use crate::data::notify_enum::NotifyType;
use fltk::app;
use fltk::enums::Event;
use fltk::tree::Tree;
use fltk::{
    enums::Color,
    group::{Pack, PackType},
    input::MultilineInput,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
};

use crate::data::COLUMN_COUNT;
use crate::logic::json_handle;

pub(crate) struct ContentPanel {
    panel: Box<Pack>,
    left: Box<MultilineInput>,
    center: Box<Tree>,
    right: Box<MultilineInput>,
}

impl ContentPanel {
    pub(crate) fn new_content_view(width: i32, height: i32) -> Self {
        let mut grid_pack = Pack::default().with_size(width, height);
        grid_pack.set_type(PackType::Horizontal);
        // grid_pack.set_spacing(10);

        let mut input = MultilineInput::default().with_size(width / COLUMN_COUNT, height);
        grid_pack.end();
        grid_pack.add(&input);

        let mut tree_view = JsonStructure::new(width / COLUMN_COUNT, height);
        let tree = *tree_view.get_tree();

        grid_pack.end();
        grid_pack.add(&tree);

        let mut result = MultilineInput::default().with_size(width / 3, height);
        result.set_readonly(true);
        result.set_color(Color::Gray0);
        grid_pack.end();
        grid_pack.add(&result);

        let mut right = Box::new(result);
        let mut right_box = right.clone();
        input.handle(move |i, e| match e {
            Event::Unfocus => {
                let text = &*i.value();
                if (text.len() > 10_000_000) {
                    // foot_cent: too long
                    return true;
                }
                let (s, _) = app::channel::<NotifyType>();
                s.send(NotifyType::Input(i.value()));
                // foot_left.set_label("Computing");
                let str = serde_json::from_str(text);
                match str {
                    Ok(json) => {
                        right_box.set_value(&*json_handle::pretty_json(&json));
                        // foot_cent.set_label("Normal");
                    }
                    Err(_) => {
                        right_box.set_value("");
                        // foot_cent.set_label("Illegal input");
                    }
                }
                // foot_left.set_label(READY);
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
