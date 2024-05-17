use chrono::{DateTime, Local};
use strum::AsRefStr;
use crate::data::stack::Stack;

#[derive(Clone)]
pub(crate) enum NotifyType {
    Resize(i32, i32),
    Input(String),
    Status(ComputeStatus),
    Result(ComputeResult),
    SelectedTree(Vec<Stack<String>>, DateTime<Local>),

}

#[derive(Clone, AsRefStr)]
pub(crate) enum ComputeStatus {
    Preparing,
    Waiting,// waiting for user's input continuously, up to 2 seconds
    Computing,
    Ready,
}

#[derive(Clone, AsRefStr)]
pub(crate) enum ComputeResult {
    Normal,
    Error(String),
}