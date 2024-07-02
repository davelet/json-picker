use std::sync::{Arc, RwLock};
use chrono::{DateTime, Local};

pub(crate) struct HaltWaitingStatusTaskParam {
    halt_time: Option<DateTime<Local>>,
    update_count: Arc<RwLock<i32>>,
}

impl HaltWaitingStatusTaskParam {
    pub(crate) fn new(halt_time: Option<DateTime<Local>>) -> Self {
        Self {
            halt_time,
            update_count: Arc::new(RwLock::new(0)),
        }
    }


    pub fn halt_time(&self) -> Option<DateTime<Local>> {
        self.halt_time
    }
    pub fn update_count(&self) -> Arc<RwLock<i32>> {
        self.update_count.clone()
    }

    pub(crate) fn with_time(&mut self, time: DateTime<Local>) {
        self.halt_time = Some(time);
    }
}