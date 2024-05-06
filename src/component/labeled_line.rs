use std::cell::RefCell;
use std::rc::Rc;

use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetExt};

use crate::data::COLUMN_COUNT;

const SIZE_DISPLAY: &str = "Window:";
const HEADER_HEIGHT: i32 = 20;
const FOOTER_HEIGHT: i32 = 20;

pub(crate) struct LabeledLine {
    content: Rc<RefCell<Pack>>,
    children: Rc<Vec<Box<Frame>>>,
    width: i32,
    height: i32,
}

impl LabeledLine {
    pub fn new(width: i32, height: i32) -> Self {
        let mut pack = Pack::default().with_size(width, height);
        pack.set_type(PackType::Horizontal);
        let left = Frame::default().with_size(width / COLUMN_COUNT, height);
        let center = Frame::default().with_size(width / COLUMN_COUNT, height);
        let right = Frame::default().with_size(width / COLUMN_COUNT, height);
        pack.end();
        let children = vec![Box::new(left), Box::new(center), Box::new(right)];
        LabeledLine {
            content: Rc::new(RefCell::new(pack)),
            children: Rc::new(children),
            width,
            height,
        }
    }

    pub(crate) fn content(&self) -> Rc<RefCell<Pack>> {
        self.content.clone()
    }

    pub fn child(&self, index: usize) -> Box<Frame> {
        self.children[index].clone()
    }

    pub(crate) fn make_header(width: i32) -> Self {
        let line = Self::new(width, HEADER_HEIGHT);
        line.child(0).set_label("input area");
        line.child(1).set_label("structure area");
        line.child(2).set_label("view area");
        line
    }

    pub(crate) fn init_footer(width: i32) -> Self {
        let line = Self::new(width, FOOTER_HEIGHT);
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

    pub(crate) fn resize_with_parent_width(&mut self, parent_w: i32) {
        self.content().borrow_mut().set_size(parent_w, self.height);
        self.child(0).set_size(parent_w / COLUMN_COUNT, self.height);
        self.child(1).set_size(parent_w / COLUMN_COUNT, self.height);
        self.child(2).set_size(parent_w / COLUMN_COUNT, self.height);
    }
}
