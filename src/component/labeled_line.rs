use std::sync::{Arc, Mutex};

use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetExt};

use crate::data::constants::{COLUMN_COUNT, FOOTER_HEIGHT, HEADER_HEIGHT};
use crate::data::notify_enum::{ComputeResult, ComputeStatus};

const SIZE_DISPLAY: &str = "Window:";

pub(crate) struct LabeledLine {
    whole_line: Arc<Mutex<Pack>>,
    children: Arc<Vec<Box<Frame>>>,
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
            whole_line: Arc::new(Mutex::new(pack)),
            children: Arc::new(children),
            height,
        }
    }

    pub(crate) fn get_whole_line(&self) -> Arc<Mutex<Pack>> {
        self.whole_line.clone()
    }

    fn child(&self, index: usize) -> Box<Frame> {
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
        line.child(0).set_label(ComputeStatus::Preparing.as_ref());
        line.child(1).set_label(ComputeResult::Normal.as_ref());

        line
    }

    pub(crate) fn get_height(&self) -> i32 {
        self.height
    }

    pub(crate) fn show_window_size(&self, width: i32, height: i32) {
        self.child(2).set_label(&*format!("{} {width} x {height}", SIZE_DISPLAY))
    }

    pub(crate) fn resize_with_parent_width(&self, parent_w: i32) {
        self.get_whole_line().lock().unwrap().set_size(parent_w, self.height);
        self.child(0).set_size(parent_w / COLUMN_COUNT, self.height);
        self.child(1).set_size(parent_w / COLUMN_COUNT, self.height);
        self.child(2).set_size(parent_w / COLUMN_COUNT, self.height);
    }
    
    pub(crate) fn set_status(&self, st: &ComputeStatus) {
        self.child(0).set_label(st.as_ref())
    }
    
    pub(crate) fn set_result(&self, result: &ComputeResult) {
        let result = match result {
            ComputeResult::Normal => {ComputeResult::Normal.as_ref()}
            ComputeResult::Error(e) => e
        };
        self.child(1).set_label(result);
    }
}
