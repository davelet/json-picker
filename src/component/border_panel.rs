use fltk::{
    group::{Pack, PackType},
    prelude::{GroupExt, WidgetExt},
};

use crate::data::singleton::FOOT_SHOW;

use super::{labeled_line::LabeledLine, main_panel::ContentPanel};

pub(crate) struct WholeViewPanel {
    panel: Box<Pack>,
    header: Box<LabeledLine>,
    content: Box<ContentPanel>,
}

impl WholeViewPanel {
    pub(crate) fn new(width: i32, height: i32) -> Self {
        let mut whole_view = Pack::default().with_size(width, height);
        whole_view.set_type(PackType::Vertical);

        let line = LabeledLine::make_header(width);
        whole_view.add(&*(line.get_whole_line().lock().unwrap()));

        let foot = FOOT_SHOW.lock().unwrap();
        (*foot).show_window_size(width, height);

        // whole_view.end();

        let double_line_height = line.get_height() + foot.get_height();
        let grid_pack = ContentPanel::new(width, height - double_line_height);

        // whole_view.end();
        whole_view.add(&*grid_pack.get_panel());

        // whole_view.end();
        whole_view.add(&*(foot.get_whole_line().lock().unwrap()));

        WholeViewPanel {
            panel: Box::new(whole_view),
            header: Box::new(line),
            content: Box::new(grid_pack),
        }
    }

    pub(crate) fn get_panel(&self) -> Box<Pack> {
        self.panel.clone()
    }
    pub(crate) fn resize_with_auto_detect_size(&mut self) {
        let p = *self.get_panel();
        (*self.header).resize_with_parent_width(p.width());

        let footer_guard = FOOT_SHOW.lock().unwrap();
        (*footer_guard).resize_with_parent_width(p.width());
        (*footer_guard).show_window_size(p.width(), p.height());

        let margin = self.header.get_height() + footer_guard.get_height();
        (*self.content).resize_with_parent_size(p.width(), p.height() - margin);
    }
}
