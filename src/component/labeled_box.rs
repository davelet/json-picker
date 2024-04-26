use std::rc::Rc;

use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetExt};

const COLUMN_COUNT: i32 = 3;
const SIZE_DISPLAY: &str = "Window:";

pub(crate) struct LabeledLine {
    content: Rc<Pack>,
    children: Rc<Vec<Box<Frame>>>,
    width: i32,
    height: i32,
}

impl LabeledLine {
    pub fn new(width: i32, height: i32, content_size: i32) -> Self {
        let mut pack = Pack::default().with_size(width, height);
        pack.set_type(PackType::Horizontal);
        let left = Frame::default().with_size(width / content_size, height);
        let center = Frame::default().with_size(width / content_size, height);
        let right = Frame::default().with_size(width / content_size, height);
        pack.end();
        let children = vec![Box::new(left), Box::new(center), Box::new(right)];
        LabeledLine {
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
        self.children[index].clone()
    }

    pub(crate) fn make_header(width: i32, height: i32) -> Self {
        let line = Self::new(width, height, COLUMN_COUNT);
        line.child(0).set_label("input area");
        line.child(1).set_label("structure area");
        line.child(2).set_label("view area");
        line
    }

    pub(crate) fn init_footer(width: i32, height: i32) -> Self {
        let line = Self::new(width, height, COLUMN_COUNT);
        line.child(0).set_label("Ready");
        line.child(1).set_label("Normal");
        line
    }

    pub(crate) fn get_width(&self) -> i32 {
        self.width
    }
    pub(crate) fn get_height(&self) -> i32 {
        self.height
    }
    
    pub(crate) fn display_size(&self, width: i32, height: i32) {
        self.child(2).set_label(&*format!("{SIZE_DISPLAY} {width} x {height}"))
    }
}
