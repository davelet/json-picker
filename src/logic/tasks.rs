use std::sync::{Arc, RwLock};
use std::thread;

use chrono::{DateTime, Local};
use serde_json::Value;

use crate::data::constants::TREE_LABEL_SPLITTER;
use crate::data::notify_enum::{ComputeStatus, NotifyType};
use crate::data::singleton::{CHANNEL, GLOBAL_JSON};
use crate::data::stack::Stack;
use crate::data::task_bo::HaltWaitingStatusTaskParam;
use crate::logic::tasks::TaskStatus::{Initialed, Pending, Processing};

/// task trait.
/// generic param is the type of task data
pub(crate) trait Task<D> {
    fn new() -> Self;
    fn before_execute(&mut self, data: D) -> bool;
    async fn execute(&mut self, data: D);
    fn after_execute(&mut self, data: D);
    fn status(&self) -> &TaskStatus;
}

#[derive(Clone)]
enum TaskStatus {
    Initialed,
    Pending,
    Processing,
    Done,
    Error(String),
}

pub(crate) struct ComputeOnSelectedTask {
    status: TaskStatus,
    selected_path: Vec<Stack<String>>,
}

impl Task<Vec<Stack<String>>> for ComputeOnSelectedTask {
    fn new() -> Self {
        ComputeOnSelectedTask {
            selected_path: vec![],
            status: Initialed,
        }
    }

    fn before_execute(&mut self, data: Vec<Stack<String>>) -> bool {
        self.selected_path = data;
        self.status = Pending;
        true
    }

    async fn execute(&mut self, _data: Vec<Stack<String>>) {
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

    fn after_execute(&mut self, data: Vec<Stack<String>>) {}

    fn status(&self) -> &TaskStatus {
        &self.status
    }
}

pub(crate) struct HaltWaitingStatusTask {
    status: TaskStatus,
    param: HaltWaitingStatusTaskParam,
}

impl Task<HaltWaitingStatusTaskParam> for HaltWaitingStatusTask {
    fn new() -> Self {
        HaltWaitingStatusTask {
            param: HaltWaitingStatusTaskParam::new(None),
            status: Initialed,
        }
    }

    fn before_execute(&mut self, data: HaltWaitingStatusTaskParam) -> bool {
        let x = self.param.update_count();
        let i = x.write();
        if let Ok(mut i) = i {
            if *i > 10 { return false; } // todo to reset app

            let t = data.halt_time().unwrap();
            self.param.with_time(t);
            *i = *i + 1;
            return true;
        }
        false
    }

    async fn execute(&mut self, data: HaltWaitingStatusTaskParam) {
        let arc = self.param.update_count();
        // thread::sleep(Duration::from_secs(2));// todo time diff

        let wt_lock = arc.write();
        if let Ok(mut i) = wt_lock {
            let end_time = self.param.halt_time().unwrap();
            let time = data.halt_time().unwrap();
            if end_time != time && end_time.gt(&Local::now()) {
                return;
            }
            *i = 0;
            CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Computing));
        }
    }

    fn after_execute(&mut self, data: HaltWaitingStatusTaskParam) {
    }

    fn status(&self) -> &TaskStatus {
        &self.status
    }
}