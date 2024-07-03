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

pub(crate) struct AppWindowLocationTaskParam {
    x: i64, // location x coordinate
    y: i64, // location y coordinate
    w: i64, // size width
    h: i64, // size height
}

impl AppWindowLocationTaskParam {
    pub(crate) fn new(x: i64, y: i64, w: i64, h: i64) -> Self {
        Self { x, y, w, h }
    }


    pub fn x(&self) -> i64 {
        self.x
    }
    pub fn y(&self) -> i64 {
        self.y
    }
    pub fn w(&self) -> i64 {
        self.w
    }
    pub fn h(&self) -> i64 {
        self.h
    }
}


impl PartialEq<AppWindowLocationTaskParam> for &AppWindowLocationTaskParam {
    fn eq(&self, other: &AppWindowLocationTaskParam) -> bool {
        self.x() == other.x() &&
            self.y() == other.y() &&
            self.w() == other.w() &&
            self.h() == other.h()
    }
}