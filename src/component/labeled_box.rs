use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetExt};

pub(crate) struct LabeledBox {
    content: &'static Pack,
    children: Vec<&'static Frame>,
    width: i32,
    height: i32,
}

impl LabeledBox {
    pub fn new(width: i32, height: i32, content_size: i32) -> Self {
        let mut content = Pack::default().with_size(width, height);
        content.set_type(PackType::Horizontal);
        let left = Frame::default().with_size(width / content_size, height);
        let center = Frame::default().with_size(width / content_size, height);
        let right = Frame::default().with_size(width / content_size, height);
        content.end();
        let children = vec![&left, &center, &right];
        LabeledBox { content: &content, children, width, height }
    }
}