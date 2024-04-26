use std::rc::Rc;

use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetExt};

pub(crate) struct LabeledBox {
    content: Rc<Pack>,
    children: Rc<Vec<Box<Frame>>>,
    width: i32,
    height: i32,
}

impl LabeledBox {
    pub fn new(width: i32, height: i32, content_size: i32) -> Self {
        let mut pack = Pack::default().with_size(width, height);
        pack.set_type(PackType::Horizontal);
        let left = Frame::default().with_size(width / content_size, height);
        let center = Frame::default().with_size(width / content_size, height);
        let right = Frame::default().with_size(width / content_size, height);
        pack.end();
        let children = vec![Box::new(left), Box::new(center), Box::new(right)];
        LabeledBox {
            content: Rc::new(pack),
            children: Rc::new(children),
            width,
            height,
        }
    }

    pub(crate) fn content(&self) -> Rc<Pack> {
        self.content.clone()
    }

    pub fn child(&self, index: usize) -> Box<Frame> {
        // if index < 0 {
        //     return None;
        // }
        // if index >= self.children.len() {
        //     return None;
        // }

        // Some(self.children[index])
        self.children[index].clone()
    }
}
