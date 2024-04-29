use fltk::{
    group::{Pack, PackType},
    prelude::{GroupExt, WidgetBase, WidgetExt},
};
use super::{labeled_line::LabeledLine, main_panel::ContentPanel};

pub(crate) struct WholeViewPanel {
    panel: Box<Pack>,
    header: Box<LabeledLine>,
    footer: Box<LabeledLine>,
    content: Box<ContentPanel>
}

impl WholeViewPanel {
    pub(crate) fn new_whole_view(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut whole_view = Pack::new(0, 0, width, height, "");
        whole_view.set_type(PackType::Vertical);

        let line = LabeledLine::make_header(width);
        let foot = LabeledLine::init_footer(width);
        foot.display_size(width, height);

        whole_view.end();
        whole_view.add(&*line.content().borrow_mut());

        let double_line_height = line.get_height() + foot.get_height();
        let grid_pack = ContentPanel::new_content_view(0, line.get_height(), width, height - double_line_height);

        whole_view.end();
        whole_view.add(&*grid_pack.get_panel());

        whole_view.end();
        whole_view.add(&*foot.content().borrow_mut());

        WholeViewPanel {
            panel: Box::new(whole_view),
            header: Box::new(line),
            footer: Box::new(foot),
            content: Box::new(grid_pack)
        }
    }

    pub(crate) fn get_panel(&self) -> Box<Pack> {
        self.panel.clone()
    }

    pub(crate) fn resize_with_ratio(&mut self, width_ratio: f32, height_ratio: f32) {
        let p = *self.get_panel();
        println!("o = {} {}, n = {} {}", p.width(), p.height(), width_ratio, height_ratio);
        (*self.header).resize_with_ratio(p.width(), p.height(), width_ratio, height_ratio);
        (*self.footer).resize_with_ratio(p.width(), p.height(), width_ratio, height_ratio);
        let margin = self.header.get_height() + self.footer.get_height();
        (*self.content).resize_with_ratio(p.width(), p.height() - margin, width_ratio, height_ratio);
    }

}
