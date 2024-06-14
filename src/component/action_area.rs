use std::sync::{Arc, Mutex};

use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

use crate::data::singleton::ACTION_BTNS;

pub(crate) struct ActionArea {
    area: Arc<Mutex<Pack>>,
}
impl ActionArea {
    pub(crate) fn new(width: i32, height: i32) -> Self {

        let mut pack = Pack::default().with_size(width, height);
        pack.set_type(PackType::Horizontal);
        let btns = ACTION_BTNS.lock().unwrap();
        pack.end();
        ActionArea {
            area: Arc::new(Mutex::new(pack))
        }
    }

    pub fn area(&self) -> Arc<Mutex<Pack>> {
        self.area.clone()
    }
}