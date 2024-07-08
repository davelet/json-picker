use std::thread;

use chrono::Local;
use serde_json::Value;

use crate::data::constants::TREE_LABEL_SPLITTER;
use crate::data::notify_enum::{ComputeStatus, NotifyType};
use crate::data::singleton::{CHANNEL, GLOBAL_JSON};
use crate::data::stack::Stack;
use crate::data::task_bo::{AppWindowLocationTaskParam, HaltWaitingStatusTaskParam};
use crate::logic::app_startup::{store_location, store_snapshot};
use crate::logic::workers::ui_tasks::TaskStatus::{Initialed, Pending};

/// task trait.
/// generic param is the type of task data
pub(crate) trait UiTask<D> {
    fn new() -> Self;
    fn before_execute(&mut self, data: D) -> bool;
    fn execute(&mut self, data: D);
    // fn after_execute(&mut self, data: D);
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
    selected_path: Vec<Stack<String>>,
    status: TaskStatus
}

impl UiTask<Vec<Stack<String>>> for ComputeOnSelectedTask {
    fn new() -> Self {
        ComputeOnSelectedTask {
            selected_path: vec![],
            status: Initialed
        }
    }

    fn before_execute(&mut self, data: Vec<Stack<String>>) -> bool {
        if data.len() == 0 {
            return false;
        }
        self.selected_path = data;
        self.status = Pending;
        true
    }

    fn execute(&mut self, _data: Vec<Stack<String>>) {
        if let Pending = self.status {
            let mut cp = self.selected_path.clone();
            thread::spawn(move || {
                let mut guard = GLOBAL_JSON.lock().unwrap();
                let mut json = (*guard).get_mut().clone();
                let path = &mut cp[0]; // only pick the first one
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
                CHANNEL.0.clone().send(NotifyType::SelectedTree(json));
            });
        }
    }
}

pub(crate) struct HaltWaitingStatusTask {
    status: TaskStatus,
    param: HaltWaitingStatusTaskParam,
}

impl UiTask<HaltWaitingStatusTaskParam> for HaltWaitingStatusTask {
    fn new() -> Self {
        HaltWaitingStatusTask {
            param: HaltWaitingStatusTaskParam::new(None),
            status: Initialed,
        }
    }

    fn before_execute(&mut self, data: HaltWaitingStatusTaskParam) -> bool {
        match &self.status {
            Pending | TaskStatus::Processing => return false,
            TaskStatus::Error(err) => {
                println!("resorted from {err}")
            }
            _ => {}
        }
        let x = self.param.update_count();
        let x = match x.write() {
            Ok(mut i) => {
                if *i > 10 {
                    return false;
                } // todo to reset app

                let t = data.halt_time().unwrap();
                self.param.with_time(t);
                *i = *i + 1;
                self.status = TaskStatus::Pending;
                true
            }
            Err(e) => {
                self.status = TaskStatus::Error(e.to_string());
                false
            }
        };
        x
    }

    fn execute(&mut self, data: HaltWaitingStatusTaskParam) {
        self.status= TaskStatus::Processing;
        let arc = self.param.update_count();
        // thread::sleep(Duration::from_secs(2));// todo time diff

        let wt_lock = arc.write();
        match wt_lock {
            Ok(mut i) => {
                let end_time = self.param.halt_time().unwrap();
                let time = data.halt_time().unwrap();
                if end_time != time && end_time.gt(&Local::now()) {
                    return;
                }
                *i = 0;
                CHANNEL
                    .0
                    .clone()
                    .send(NotifyType::Status(ComputeStatus::Computing));
                self.status = TaskStatus::Done;
            }
            Err(e) => {self.status = TaskStatus::Error(e.to_string())},
        }
    }
}

pub(crate) struct AppWindowLocationPersistenceTask {
    status: TaskStatus,
    location: Option<AppWindowLocationTaskParam>,
}

impl UiTask<AppWindowLocationTaskParam> for AppWindowLocationPersistenceTask {
    fn new() -> Self {
        Self {
            status: Initialed,
            location: None,
        }
    }

    fn before_execute(&mut self, data: AppWindowLocationTaskParam) -> bool {
        match self.status {
            Pending | TaskStatus::Processing => return false,
            _ => {}
        }
        if let Some(p) = &self.location {
            if p == data {
                return false;
            }
        }
        self.location = Some(data);
        self.status = TaskStatus::Pending;
        true
    }

    fn execute(&mut self, _data: AppWindowLocationTaskParam) {
        self.status = TaskStatus::Processing;
        let data = self.location.as_ref().unwrap();
        store_location(data.x(), data.y(), data.w(), data.h());
        self.status = TaskStatus::Done;
    }
}

pub(crate) struct ParsedJsonStringPersistenceTask {
    status: TaskStatus,
    value: String,
}

impl UiTask<String> for ParsedJsonStringPersistenceTask {
    fn new() -> Self {
        Self {
            status: Initialed,
            value: "".into(),
        }
    }

    fn before_execute(&mut self, data: String) -> bool {
        match self.status {
            Pending | TaskStatus::Processing => false,
            _ => {
                if self.value == data {
                    return false;
                }
                self.value = data;
                self.status = Pending;
                true
            }
        }
    }

    fn execute(&mut self, _data: String) {
        self.status = TaskStatus::Processing;
        store_snapshot(&self.value);
        self.status = TaskStatus::Done;
    }
}
