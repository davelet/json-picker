use std::thread;
use chrono::{DateTime, Local};
use strum::AsRefStr;
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
    target_time: DateTime<Local>,
}

impl ComputeOnSelectedTask {
    pub(crate) fn new() -> Self {
        ComputeOnSelectedTask {
            status: TaskStatus::Initialed.as_ref().to_string(),
            selected_path: vec![],
            target_time: Local::now()
        }
    }
    pub(crate) fn setup(&mut self, paths: Vec<Stack<String>>, time: DateTime<Local>) {
        self.selected_path = paths;
        self.target_time = time;
    }

    fn exec(&self) {
        thread::spawn(|| {
            let tt = self.target_time.clone();
            let diff = self.target_time.signed_duration_since(Local::now());
            if diff.num_milliseconds() > 0 {
                thread::sleep_ms(diff.num_milliseconds() as u32);
            }
            if tt == self.target_time {
                // 不太好 依赖状态栏，状态栏从 waiting 变为 compute 再计算
            }
        });
    }
}