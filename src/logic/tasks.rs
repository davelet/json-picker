use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Local};
use serde_json::Value;
use strum::AsRefStr;
use crate::data::notify_enum::{ComputeStatus, NotifyType};
use crate::data::singleton::{CHANNEL, GLOBAL_JSON};
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
        println!("selected from {} to {}", self.selected_path.len(), paths.len());
        self.selected_path = paths;
    }

    pub(crate) fn compute(&self) {
        let mut cp = self.selected_path.clone();
        println!("compute selected {}", cp.len());
        if cp.len() == 0 { return; }
        thread::spawn(move || {
            println!("start >>>>>>>>>> ");
            for p in &cp {
                println!("selected {p}");
            }
            println!("end <<<<<<<<<<< ");
            let mut guard = GLOBAL_JSON.lock().unwrap();
            let mut json = (*guard).get_mut().clone();
            if cp.len() > 0 {
                let mut path = &mut cp[0];
                let mut c = path.pop();
                while let Some(ref n) = c {
                    match json {
                        Value::Object(ref j) => {
                            let np = j.get(n);
                            if let Some(vv) = np {
                                json = (*vv).clone();
                            }
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
        thread::sleep(Duration::from_secs(2));// todo time diff

        let wt_lock = arc.write();
        match wt_lock {
            Err(_) => {}
            Ok(mut i) => {
                let end_time = self.halt_time;
                println!("execute time from {} {}", &end_time, &time);
                if end_time != time && end_time.gt(&Local::now()) {
                    return;
                }
                *i = 0;
                CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Computing));
            }
        }
    }
}