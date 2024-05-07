use fltk::app;

use self::notify_enum::NotifyType;

pub(crate) mod notify_enum;
pub(crate) const APP_NAME: &str = "JSON HAND";
pub(crate) const DEFAULT_WIDTH: i32 = 800;
pub(crate) const DEFAULT_HEIGHT: i32 = 600;
pub(crate) const MIN_WIDTH: i32 = 800;
pub(crate) const MIN_HEIGHT: i32 = 600;

pub(crate) const COLUMN_COUNT: i32 = 3;
lazy_static::lazy_static! {
    pub(crate) static ref CHANNEL: (app::Sender<NotifyType>, app::Receiver<NotifyType>) = app::channel();
}