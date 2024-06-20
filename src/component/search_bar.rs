use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetExt};
use crate::data::constants::SEARCH_BAR_HEIGHT;
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
}