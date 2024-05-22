use std::cell::Cell;
use std::sync::Mutex;
use serde_json::Value;
use serde_json::Value::Null;

use crate::data::notify_enum::NotifyType;
use crate::logic::tasks::{ComputeOnSelectedTask, HaltWaitingStatusTask};

lazy_static::lazy_static! {
    pub(crate) static ref CHANNEL: (fltk::app::Sender<NotifyType>, fltk::app::Receiver<NotifyType>) = fltk::app::channel();
    pub(crate) static ref STATUS_TASK: (Mutex<HaltWaitingStatusTask>,) = (Mutex::new(HaltWaitingStatusTask::new()), );
    pub(crate) static ref COMPUTE_TASK: Mutex<ComputeOnSelectedTask> = Mutex::new(ComputeOnSelectedTask::new());
    pub(crate) static ref GLOBAL_JSON: Mutex<Cell<Value>> = Mutex::new(Cell::new(Null));
}
