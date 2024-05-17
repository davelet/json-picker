use std::cell::Cell;
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
        thread::spawn(|| {
        });
    }
}

pub(crate) struct HaltWaitingStatusTask {
    halt_time: DateTime<Local>,
    update_count: Cell<i32>,
}

impl HaltWaitingStatusTask {
    pub(crate) fn new() -> Self {
        HaltWaitingStatusTask {
            halt_time: Local::now(),
            update_count: Cell::new(0),
        }
    }

    fn reset(&mut self) {
        self.halt_time = Local::now();
    }

    pub(crate) fn set_halt_time(&mut self, time: DateTime<Local>) {
        let i = self.update_count.get();
        if self.update_count.get_mut() > &mut 10 { return; } // todo to reset app

        self.halt_time = time;
        self.update_count.set(i + 1);
        thread::spawn(move || {
            let start_time = time;
            thread::sleep(Duration::from_secs(2));// todo time diff

            if self.update_count.get() > 0 {
                self.update_count.set(self.update_count.get() - 1);
            }
            let end_time = self.halt_time;
            if end_time != start_time && end_time.gt(&Local::now()) {
                return;
            }
            CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Computing));
        });
    }
}