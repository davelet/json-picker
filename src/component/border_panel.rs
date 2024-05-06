use fltk::{
    app, group::{Pack, PackType}, prelude::{GroupExt, WidgetExt}
};

use crate::data::notify_enum::NotifyType;

use super::{labeled_line::LabeledLine, main_panel::ContentPanel};

pub(crate) struct WholeViewPanel {
    panel: Box<Pack>,
    header: Box<LabeledLine>,
    footer: Box<LabeledLine>,
    content: Box<ContentPanel>,
}

impl WholeViewPanel {
    pub(crate) fn new_whole_view(width: i32, height: i32) -> Self {
        let mut whole_view = Pack::default().with_size(width, height);
        whole_view.set_type(PackType::Vertical);

        let line = LabeledLine::make_header(width);
        let foot = LabeledLine::init_footer(width);
        foot.display_size(width, height);

        whole_view.end();
        whole_view.add(&*line.content().borrow_mut());

        let double_line_height = line.get_height() + foot.get_height();
        let grid_pack = ContentPanel::new_content_view(width, height - double_line_height);

        whole_view.end();
        whole_view.add(&*grid_pack.get_panel());

        whole_view.end();
        whole_view.add(&*foot.content().borrow_mut());

        app::add_idle(|| {
            let (s, r) = app::channel::<NotifyType>();
            let rc = r.recv();
            match rc {
                Some(nt) => {
                    match nt {
                        NotifyType::Input(t) => {
                            println!("2342 {t}", )
                        }
                        _ => {}
                    }
                    
                }
                _ => {}
            }
        });

        WholeViewPanel {
            panel: Box::new(whole_view),
            header: Box::new(line),
            footer: Box::new(foot),
            content: Box::new(grid_pack),
        }
    }

    pub(crate) fn get_panel(&self) -> Box<Pack> {
        self.panel.clone()
    }

    pub(crate) fn resize_with_auto_detect_size(&mut self) {
        let p = *self.get_panel();
        (*self.header).resize_with_parent_width(p.width());
        (*self.footer).resize_with_parent_width(p.width());
        (*self.footer).display_size(p.width(), p.height());
        let margin = self.header.get_height() + self.footer.get_height();
        (*self.content).resize_with_parent_size(p.width(), p.height() - margin);
    }
}
