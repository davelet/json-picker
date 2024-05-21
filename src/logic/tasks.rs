use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Local};
use serde_json::Value;
use strum::AsRefStr;
use crate::data::notify_enum::{ComputeStatus, NotifyType};
use crate::data::singleton::CHANNEL;
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
        println!("compute {:?}", &self.selected_path);
        thread::spawn(|| {
            CHANNEL.0.clone().send(NotifyType::SelectedTree(Value::Bool(true)));
            CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Ready));
        });
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

    // fn reset(&mut self) {
    //     self.halt_time = Local::now();
    // }

    pub(crate) fn set_halt_time(&mut self, time: DateTime<Local>) -> bool {
        let i = self.update_count.write();
        match i {
            Err(_) => { return false; }
            Ok(mut i) => {
                if *i > 10 { return false; } // todo to reset app

                self.halt_time = time;
                *i = *i + 1;
            }
        }
        true
    }

    pub(crate) fn exec(&self, time: DateTime<Local>) {
        let arc = self.update_count.clone();
        let start_time = time;
        thread::sleep(Duration::from_secs(2));// todo time diff

        let wt_lock = arc.write();
        match wt_lock {
            Err(_) => {}
            Ok(mut i) => {
                let end_time = self.halt_time;
                if end_time != time && end_time.gt(&Local::now()) {
                    return;
                }
                *i = 0;
                CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Computing));
            }
        }
    }
}