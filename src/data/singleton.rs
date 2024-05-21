use std::cell::RefCell;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use chrono::Local;

use crate::data::notify_enum::NotifyType;
use crate::logic::tasks::HaltWaitingStatusTask;

lazy_static::lazy_static! {
    pub(crate) static ref CHANNEL: (fltk::app::Sender<NotifyType>, fltk::app::Receiver<NotifyType>) = fltk::app::channel();
    pub(crate) static ref STATUS_TASK: (Mutex<RefCell<HaltWaitingStatusTask>>,) = (Mutex::new(RefCell::new(HaltWaitingStatusTask::new())), );
}

pub(crate) fn get_status_task(timeout: i64) -> Result<HaltWaitingStatusTask, String> {
    let start = Local::now();
    let mut task = STATUS_TASK.0.lock();
    while let Err(_) = task {
        thread::sleep(Duration::from_millis(10));
        let last = Local::now() - start;
        if last.num_milliseconds() > timeout {
            return Err("timeout".to_string());
        }
        task = STATUS_TASK.0.lock();
    }
    Ok(**task.unwrap())
}