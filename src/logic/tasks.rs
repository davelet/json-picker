use std::sync::{Arc, RwLock};
use std::thread;

use chrono::{DateTime, Local};
use serde_json::Value;

use crate::data::constants::TREE_LABEL_SPLITTER;
use crate::data::notify_enum::{ComputeStatus, NotifyType};
use crate::data::singleton::{CHANNEL, GLOBAL_JSON};
use crate::data::stack::Stack;

// #[derive(Clone, AsRefStr)]
// enum TaskStatus {
//     Initialed,
//     Pending,
//     Processing,
//     Done,
//     Error(String),
// }

pub(crate) struct ComputeOnSelectedTask {
    // status: String,
    selected_path: Vec<Stack<String>>,
}

impl ComputeOnSelectedTask {
    pub(crate) fn new() -> Self {
        ComputeOnSelectedTask {
            // status: TaskStatus::Initialed.as_ref().to_string(),
            selected_path: vec![],
        }
    }
    pub(crate) fn setup(&mut self, paths: Vec<Stack<String>>) {
        self.selected_path = paths;
    }

    pub(crate) fn compute(&self) {
        let mut cp = self.selected_path.clone();
        if cp.len() == 0 { return; }
        thread::spawn(move || {
            let mut guard = GLOBAL_JSON.lock().unwrap();
            let mut json = (*guard).get_mut().clone();
            if cp.len() > 0 {
                let mut path = &mut cp[0];
                let mut c = path.pop();
                while let Some(ref n) = c {
                    if !n.contains(TREE_LABEL_SPLITTER) {
                        c = path.pop();
                        continue;
                    }
                    let field = n.split_once(TREE_LABEL_SPLITTER).unwrap().0;
                    match json {
                        Value::Object(ref map) => {
                            let np = map.get(field);
                            if let Some(vv) = np {
                                json = (*vv).clone();
                            }
                        }
                        Value::Array(ref arr) => {
                            let index = field.parse::<usize>().unwrap();
                            let value = &arr[index];
                            json = value.clone();
                        }
                        _ => {}
                    }
                    c = path.pop();
                }
            }
            CHANNEL.0.clone().send(NotifyType::SelectedTree(json));
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
        // thread::sleep(Duration::from_secs(2));// todo time diff

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