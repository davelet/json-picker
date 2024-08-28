use std::sync::{Arc, Mutex};

use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetExt};
use crate::data::constants::ACTION_BUTTON_COUNT;

use crate::data::singleton::ACTION_BTNS;

pub(crate) struct ActionArea {
    area: Arc<Mutex<Pack>>,
}
impl ActionArea {
    pub(crate) fn new(width: i32, height: i32) -> Self {

        let mut pack = Pack::default().with_size(width, height);
        pack.set_type(PackType::Horizontal);
        let _btns = ACTION_BTNS.lock().unwrap();
        pack.end();
        ActionArea {
            area: Arc::new(Mutex::new(pack))
        }
    }

    pub(crate) fn area(&self) -> Arc<Mutex<Pack>> {
        self.area.clone()
    }
    pub(crate) fn get_height(&self) -> i32 {
        self.area().lock().unwrap().h()
    }

    pub(crate) fn resize(&self, width: i32) {
        let mut btns = ACTION_BTNS.lock().unwrap();
        for btn in &mut *btns {
            btn.set_size((width as f64 / ACTION_BUTTON_COUNT as f64).round() as i32, btn.h());
        }
    }
}