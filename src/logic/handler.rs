use fltk::app;
use fltk::enums::Event;
use fltk::prelude::WidgetBase;
use fltk::window::Window;
use crate::data::constants::START_TIMEOUT;
use crate::data::notify_enum::{ComputeStatus, NotifyType};
use crate::data::singleton::{CHANNEL, WHOLE_VIEW};

pub(crate) fn make_ready() {
    app::add_timeout3(START_TIMEOUT, |_| {CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Ready));} );
}
pub(crate) fn window_resize(window: &mut Window) {
    let mut whole_view = WHOLE_VIEW.lock().unwrap();
    window.handle(move |_, e| match e {
        Event::Resize => {
            whole_view.resize_with_auto_detect_size();
            true
        }
        _ => false,
    });
}