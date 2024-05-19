use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Local};
use strum::AsRefStr;
use crate::data::constants::CHANNEL;
use crate::data::notify_enum::{ComputeStatus, NotifyType};
use crate::data::stack::Stack;

#[derive(Clone, AsRefStr)]
enum TaskStatus {
    Initialed,
    Pending,
    Processing,
    Done,
    Error(String),
}

pub(crate) struct ComputeOnSelectedTask {
    status: String,
    selected_path: Vec<Stack<String>>,
}

impl ComputeOnSelectedTask {
    pub(crate) fn new() -> Self {
        ComputeOnSelectedTask {
            status: TaskStatus::Initialed.as_ref().to_string(),
            selected_path: vec![],
        }
    }
    pub(crate) fn setup(&mut self, paths: Vec<Stack<String>>) {
        self.selected_path = paths;
    }

    pub(crate) fn compute(&self) {
        thread::spawn(|| {});
    }
}

pub(crate) struct HaltWaitingStatusTask {
    halt_time: DateTime<Local>,
    update_count: Arc<RwLock<i32>>,
}

impl HaltWaitingStatusTask {
    pub(crate) fn new() -> Self {
        HaltWaitingStatusTask {
            halt_time: Local::now(),
            update_count: Arc::new(RwLock::new(0)),
        }
    }

    fn reset(&mut self) {
        self.halt_time = Local::now();
    }

    pub(crate) fn set_halt_time(&mut self, time: DateTime<Local>) {
        let i = self.update_count.write();
        match i {
            Err(_) => { return; }
            Ok(mut i) => {
                if *i > 10 { return; } // todo to reset app

                self.halt_time = time;
                *i = *i + 1;
                let uc =self.update_count.clone();
                thread::spawn(move || {
                    let start_time = time;
                    thread::sleep(Duration::from_secs(2));// todo time diff

                    let wt_lock = uc.write();

                    // match wt_lock {
                    //     Err(_) => {
                    //         // if get lock failed (means the valued locked by another thread), just try to get it and decrease the value
                    //         thread::spawn(|| {
                    //             let mut lock = self.update_count.write();
                    //             while let Err(_) = lock {
                    //                 lock = self.update_count.write();
                    //             }
                    //             let mut i = lock.unwrap();
                    //             *i = *i - 1;
                    //         })
                    //     }
                    //     Ok(i) => {
                    //         let c = *i;
                    //         if c > 0 {c = c - 1; }
                    //         // let end_time = self.halt_time;
                    //         // if end_time != start_time && end_time.gt(&Local::now()) {
                    //         //     return;
                    //         // }
                    //         // CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Computing));
                    //     }
                    // }
                });
            }
        }
    }
}