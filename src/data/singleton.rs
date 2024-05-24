use std::cell::Cell;
use std::sync::{Arc, Mutex};
use fltk::prelude::WidgetExt;

use serde_json::Value;
use serde_json::Value::Null;
use crate::component::border_panel::WholeViewPanel;
use crate::component::labeled_line::LabeledLine;

use crate::component::window::AppWindow;
use crate::data::constants::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::data::notify_enum::NotifyType;
use crate::logic::tasks::{ComputeOnSelectedTask, HaltWaitingStatusTask};

lazy_static::lazy_static! {
    pub(crate) static ref APP_WINDOW: Mutex<AppWindow> = Mutex::new(AppWindow::new());
    pub(crate) static ref WHOLE_VIEW: Mutex<WholeViewPanel> = Mutex::new(WholeViewPanel::new(DEFAULT_WIDTH, DEFAULT_HEIGHT));
    pub(crate) static ref FOOT_SHOW: Arc<Mutex<LabeledLine>> = Arc::new(Mutex::new(LabeledLine::init_footer(DEFAULT_WIDTH)));

    pub(crate) static ref CHANNEL: (fltk::app::Sender<NotifyType>, fltk::app::Receiver<NotifyType>) = fltk::app::channel();
    pub(crate) static ref STATUS_TASK: (Mutex<HaltWaitingStatusTask>,) = (Mutex::new(HaltWaitingStatusTask::new()), );
    pub(crate) static ref COMPUTE_TASK: Mutex<ComputeOnSelectedTask> = Mutex::new(ComputeOnSelectedTask::new());
    pub(crate) static ref GLOBAL_JSON: Mutex<Cell<Value>> = Mutex::new(Cell::new(Null));
}
