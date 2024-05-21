use std::cell::RefCell;
use std::sync::Mutex;

use crate::data::notify_enum::NotifyType;
use crate::logic::tasks::{ComputeOnSelectedTask, HaltWaitingStatusTask};

lazy_static::lazy_static! {
    pub(crate) static ref CHANNEL: (fltk::app::Sender<NotifyType>, fltk::app::Receiver<NotifyType>) = fltk::app::channel();
    pub(crate) static ref STATUS_TASK: (Mutex<HaltWaitingStatusTask>,) = (Mutex::new(HaltWaitingStatusTask::new()), );
    pub(crate) static ref COMPUTE_TASK: Mutex<ComputeOnSelectedTask> = Mutex::new(ComputeOnSelectedTask::new());
}
