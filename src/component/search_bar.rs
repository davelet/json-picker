use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetExt};
use crate::data::constants::{ACTION_BUTTON_HEIGHT, SEARCH_BAR_HEIGHT, SEARCH_BTN_WIDTH};
use crate::data::singleton::{TREE_SEARCH_BOX, TREE_SEARCH_BTN};

pub(crate) struct SearchBar {
    bar: Box<Pack>
}

impl SearchBar {
    pub(crate) fn new(w: i32) -> Self {
        let mut search_bar = Pack::default().with_size(w, SEARCH_BAR_HEIGHT);
        search_bar.set_type(PackType::Horizontal);
        let input = TREE_SEARCH_BOX.lock().unwrap();
        let sea_btn = TREE_SEARCH_BTN.lock().unwrap();
        search_bar.add(&*input);
        search_bar.add(&*sea_btn);
        search_bar.end();
        SearchBar {
            bar: Box::new(search_bar)
        }
    }

    pub(crate) fn get_bar(&self) -> Box<Pack> {
        self.bar.clone()
    }

    pub(crate) fn resize(&mut self, w: i32) {
        self.bar.set_size(w, ACTION_BUTTON_HEIGHT);
        let mut input = TREE_SEARCH_BOX.lock().unwrap();
        input.set_size(w - SEARCH_BTN_WIDTH, ACTION_BUTTON_HEIGHT);
    }
}