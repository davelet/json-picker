use crate::data::notify_enum::NotifyType;


pub(crate) const APP_NAME: &str = "JSON HAND";
pub(crate) const DEFAULT_WIDTH: i32 = 800;
pub(crate) const DEFAULT_HEIGHT: i32 = 600;
pub(crate) const MIN_WIDTH: i32 = 800;
pub(crate) const MIN_HEIGHT: i32 = 600;

pub(crate) const COLUMN_COUNT: i32 = 3;
pub(crate) const START_TIMEOUT: f64 = 0.5;
pub(crate) const JSON_SIZE_LIMIT: usize = 1_000_000;

lazy_static::lazy_static! {
    pub(crate) static ref CHANNEL: (fltk::app::Sender<NotifyType>, fltk::app::Receiver<NotifyType>) = fltk::app::channel();
}