use strum::AsRefStr;

#[derive(Clone)]
pub(crate) enum NotifyType {
    Resize(i32, i32),
    Input(String),
    Status(ComputeStatus),
    Result(ComputeResult),
    SelectedTree(Vec<String>),

}

#[derive(Clone, AsRefStr)]
pub(crate) enum ComputeStatus {
    Preparing,
    Computing,
    Ready,
}

#[derive(Clone, AsRefStr)]
pub(crate) enum ComputeResult {
    Normal,
    Error(String)
}