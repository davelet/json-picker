use std::rc::Rc;
use std::sync::Arc;
use std::thread;

use fltk::{
    app,
    group::{Pack, PackType},
    prelude::{GroupExt, WidgetExt},
};

use crate::data::notify_enum::ComputeStatus;
use crate::data::notify_enum::NotifyType;
use crate::data::singleton::{CHANNEL, COMPUTE_TASK, STATUS_TASK};

use super::{labeled_line::LabeledLine, main_panel::ContentPanel};

pub(crate) struct WholeViewPanel {
    panel: Box<Pack>,
    header: Box<LabeledLine>,
    footer: Arc<LabeledLine>,
    content: Box<ContentPanel>,
}

impl WholeViewPanel {
    pub(crate) fn new(width: i32, height: i32) -> Self {
        let mut whole_view = Pack::default().with_size(width, height);
        whole_view.set_type(PackType::Vertical);

        let line = LabeledLine::make_header(width);
        let foot = LabeledLine::init_footer(width);
        foot.display_size(width, height);

        whole_view.end();
        whole_view.add(&*(line.content().lock().unwrap()));

        let double_line_height = line.get_height() + foot.get_height();
        let grid_pack = ContentPanel::new(width, height - double_line_height);

        whole_view.end();
        whole_view.add(&*grid_pack.get_panel());

        whole_view.end();
        whole_view.add(&*(foot.content().lock().unwrap()));

        let footer = Arc::new(foot);
        let foot_rc = footer.clone();
        app::add_idle3(move |_| {
            let received_type = CHANNEL.1.recv();
            if let Some(nt) = received_type {
                match nt {
                    NotifyType::Status(status) => {
                        (*foot_rc).set_status(&status);
                        match status {
                            ComputeStatus::Waiting(up_time, selected_path) => {
                                let t = STATUS_TASK.0.lock();
                                if let Ok(mut t) = t {
                                    let set = t.set_halt_time(up_time);
                                    if set {
                                        let lock = COMPUTE_TASK.lock();
                                        if let Ok(mut task) = lock {
                                            task.setup(selected_path);
                                        }
                                        thread::spawn(move || {
                                            let x = STATUS_TASK.0.lock();
                                            if let Ok(x) = x {
                                                x.exec(up_time);
                                            } else {
                                                // reset app
                                            }
                                        });
                                    }
                                } else {
                                    // reset app
                                }
                            }
                            ComputeStatus::Computing => {
                                let lock = COMPUTE_TASK.lock();
                                if let Ok(task) = lock {
                                    task.compute();
                                }
                            }
                            _ => {}
                        }
                    }
                    NotifyType::Result(result) => {
                        (*foot_rc).set_result(&result);
                    }
                    _ => {}
                }
            }
        });

        WholeViewPanel {
            panel: Box::new(whole_view),
            header: Box::new(line),
            footer,
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
