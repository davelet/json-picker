use std::thread;
use fltk::app;
use fltk::app::App;
use fltk::enums::Event;
use fltk::prelude::WidgetBase;
use fltk::window::Window;

use crate::data::constants::START_TIMEOUT;
use crate::data::notify_enum::{ComputeStatus, NotifyType};
use crate::data::singleton::{CHANNEL, COMPUTE_TASK, FOOT_SHOW, STATUS_TASK, WHOLE_VIEW};

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

pub(crate) fn listen_on_events(app: &App) {
    while app.wait() {
        if let Some(nt) = CHANNEL.1.recv() {
            println!("app wait = {}", nt.as_ref());
            match nt {
                NotifyType::Status(status) => {
                    (*FOOT_SHOW.lock().unwrap()).set_status(&status);
                    println!("status {}", status.as_ref());
                    match status {
                        ComputeStatus::Waiting(up_time, selected_path) => {
                            let t = STATUS_TASK.0.lock();
                            if let Ok(mut t) = t {
                                let set = t.set_halt_time(up_time);
                                if set {
                                    let lock = COMPUTE_TASK.lock();
                                    if let Ok(mut task) = lock {
                                        task.setup(selected_path);
                                    }
                                    thread::spawn(move || {
                                        let x = STATUS_TASK.0.lock();
                                        if let Ok(x) = x {
                                            x.exec(up_time);
                                        } else {
                                            // reset app
                                        }
                                    });
                                }
                            } else {
                                // reset app
                            }
                        }
                        ComputeStatus::Computing => {
                            let lock = COMPUTE_TASK.lock();
                            if let Ok(task) = lock {
                                task.compute();
                            }
                        }
                        _ => {}
                    }
                }
                NotifyType::Result(result) => {
                    (*FOOT_SHOW.lock().unwrap()).set_result(&result);
                }
                _ => {}
            }
        }
    }
}