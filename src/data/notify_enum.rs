#[derive(Clone, Debug)]
pub(crate) enum NotifyType {
    Resize(i32, i32),
    Input(String),
    Status(String),
    Result(String),

}