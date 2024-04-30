use fltk::prelude::WidgetExt;
use fltk::tree::Tree;

pub(crate) struct JsonStructure {
    view: Box<Tree>
}

impl JsonStructure {
    pub(crate) fn new(w: i32, h: i32) -> Self {
        let mut t = Tree::default().with_size(w, h);
        JsonStructure {
            view: Box::new(t)
        }
    }

    pub(crate) fn get_tree(&self) -> Box<Tree> {
        self.view.clone()
    }
}