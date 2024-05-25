use std::cell::Cell;
use std::sync::Mutex;

use fltk::prelude::WidgetExt;
use fltk::text::{TextBuffer, TextEditor};
use serde_json::Value;
use serde_json::Value::Null;

use crate::component::border_panel::WholeViewPanel;
use crate::component::labeled_line::LabeledLine;
use crate::component::structure_tree::JsonStructure;
use crate::component::window::AppWindow;
use crate::data::constants::{COLUMN_COUNT, DEFAULT_HEIGHT, DEFAULT_WIDTH, HEADER_HEIGHT};
use crate::data::notify_enum::NotifyType;
use crate::logic::tasks::{ComputeOnSelectedTask, HaltWaitingStatusTask};

lazy_static::lazy_static! {
    pub(crate) static ref APP_WINDOW: Mutex<AppWindow> = Mutex::new(AppWindow::new());
    pub(crate) static ref WHOLE_VIEW: Mutex<WholeViewPanel> = Mutex::new(WholeViewPanel::new(DEFAULT_WIDTH, DEFAULT_HEIGHT));
    pub(crate) static ref FOOT_SHOW: Mutex<LabeledLine> = Mutex::new(LabeledLine::init_footer(DEFAULT_WIDTH));
    pub(crate) static ref RESUTL_CONTROL: Mutex<TextBuffer> = Mutex::new(TextBuffer::default());
    pub(crate) static ref TREE_VIEW: Mutex<JsonStructure> = Mutex::new(JsonStructure::new(DEFAULT_WIDTH / COLUMN_COUNT, HEADER_HEIGHT));
    pub(crate) static ref JSON_INPUT_BOX: Mutex<TextEditor> = Mutex::new(TextEditor::default().with_size(DEFAULT_WIDTH / COLUMN_COUNT, HEADER_HEIGHT));

    pub(crate) static ref CHANNEL: (fltk::app::Sender<NotifyType>, fltk::app::Receiver<NotifyType>) = fltk::app::channel();
    pub(crate) static ref STATUS_TASK: (Mutex<HaltWaitingStatusTask>,) = (Mutex::new(HaltWaitingStatusTask::new()), );
    pub(crate) static ref COMPUTE_TASK: Mutex<ComputeOnSelectedTask> = Mutex::new(ComputeOnSelectedTask::new());
    pub(crate) static ref GLOBAL_JSON: Mutex<Cell<Value>> = Mutex::new(Cell::new(Null));
}
