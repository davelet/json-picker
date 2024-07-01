use chrono::{DateTime, Local};
use serde_json::Value;
use strum::AsRefStr;

use crate::data::stack::Stack;

#[derive(Clone, AsRefStr)]
pub(crate) enum NotifyType {
    Resize(i32, i32),
    Input(String),
    Status(ComputeStatus),
    Result(ComputeResult),
    SelectedTree(Value),
    SearchTree(String),
    AppParams(AppParam)
}
unsafe impl Send for NotifyType{}
unsafe impl Sync for NotifyType{}

#[derive(Clone, AsRefStr)]
pub(crate) enum ComputeStatus {
    Preparing,
    Waiting(DateTime<Local>, Vec<Stack<String>>),// waiting for user's input continuously, up to 2 seconds
    Computing,
    Ready,
}

#[derive(Clone, AsRefStr)]
pub(crate) enum ComputeResult {
    Normal,
    Error(String),
}

#[derive(Clone, AsRefStr)]
pub(crate) enum AppParam {
    WindowSize(i32, i32, i32, i32),

}