use std::thread;
use fltk::app;
use fltk::app::App;
use fltk::enums::Event;
use fltk::prelude::{DisplayExt, WidgetBase};
use fltk::window::Window;
use serde_json::Value;

use crate::data::constants::{JSON_SIZE_LIMIT, JSON_SIZE_WARN, START_TIMEOUT};
use crate::data::notify_enum::{ComputeResult, ComputeStatus, NotifyType};
use crate::data::singleton::{CHANNEL, COMPUTE_TASK, FOOT_SHOW, GLOBAL_JSON, JSON_INPUT_BOX, RESUTL_CONTROL, STATUS_TASK, TREE_VIEW, WHOLE_VIEW};
use crate::logic::json_handle;

pub(crate) fn make_ready() {
    app::add_timeout3(START_TIMEOUT, |_| {
        CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Ready));
    });
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

pub(crate) fn handle_json_input() {
    JSON_INPUT_BOX.lock().unwrap().handle(|i, e| {
        match e {
            Event::Unfocus => {
                let buf = i.buffer().unwrap();
                let text = buf.text();
                if text.trim().len() == 0 {
                    return true;
                }
                let s = CHANNEL.0.clone();
                if text.len() > JSON_SIZE_LIMIT { // move to `settings`
                    s.send(NotifyType::Result(ComputeResult::Error(JSON_SIZE_WARN.to_string())));
                    return true;
                }
                s.send(NotifyType::Status(ComputeStatus::Computing));
                let str = serde_json::from_str::<Value>(&*text);
                match str {
                    Ok(json) => {
                        let guard = GLOBAL_JSON.lock().unwrap();
                        (*guard).set(json.clone());
                        TREE_VIEW.lock().unwrap().set_tree(&json);
                        RESUTL_CONTROL.lock().unwrap().set_text(&*json_handle::pretty_json(&json));
                        s.send(NotifyType::Result(ComputeResult::Normal));
                    }
                    Err(er) => {
                        TREE_VIEW.lock().unwrap().clear();
                        RESUTL_CONTROL.lock().unwrap().set_text("");
                        s.send(NotifyType::Result(ComputeResult::Error(er.to_string())));
                    }
                }
                s.send(NotifyType::Status(ComputeStatus::Ready));
                true
            }
            _ => false,
        }
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
                NotifyType::SelectedTree(json) => {
                    println!("rev: selected tree");
                    RESUTL_CONTROL.lock().unwrap().set_text(&*json_handle::pretty_json(&json));
                    CHANNEL.0.clone().send(NotifyType::Result(ComputeResult::Normal));
                    CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Ready));
                }
                _ => {}
            }
        }
    }
}