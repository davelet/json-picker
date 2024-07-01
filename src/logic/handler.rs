use std::thread;

use clipboard::{ClipboardContext, ClipboardProvider};
use fltk::app;
use fltk::app::App;
use fltk::enums::Event;
use fltk::prelude::{DisplayExt, InputExt, WidgetBase, WidgetExt};
use serde_json::Value;

use crate::data::constants::{ACTION_BUTTON_HEIGHT, JSON_SIZE_LIMIT, JSON_SIZE_WARN, START_TIMEOUT};
use crate::data::notify_enum::{ComputeResult, ComputeStatus, NotifyType};
use crate::data::singleton::{ACTION_BTNS, APP_WINDOW, CHANNEL, COMPUTE_TASK, FOOT_SHOW, GLOBAL_JSON, JSON_INPUT_BOX, RESUTL_VIEW, STATUS_TASK, TREE_MAIN, TREE_SEARCH_BAR, TREE_SEARCH_BOX, TREE_SEARCH_BTN, TREE_VIEW, WHOLE_VIEW};
use crate::logic::json_handle;

pub(crate) fn handle_event(app: &App) {
    window_resize();
    handle_json_input();
    listen_on_action();
    make_ready();

    listen_on_events(&app);
}

fn make_ready() {
    app::add_timeout3(START_TIMEOUT, |_| {
        CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Ready));
    });
}

fn window_resize() {
    let mut whole_view = WHOLE_VIEW.lock().unwrap();
    let mut binding = APP_WINDOW.lock().unwrap();
    let window = binding.get_window();
    window.handle(move |_, e| match e {
        Event::Resize => {
            whole_view.resize_with_auto_detect_size();
            true
        }
        _ => false,
    });
}

fn handle_json_input() {
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
                        RESUTL_VIEW.lock().unwrap().set_text(&*json_handle::pretty_json(&json));
                        s.send(NotifyType::Result(ComputeResult::Normal));
                    }
                    Err(er) => {
                        TREE_VIEW.lock().unwrap().clear();
                        RESUTL_VIEW.lock().unwrap().set_text("");
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

fn listen_on_events(app: &App) {
    while app.wait() {
        if let Some(nt) = CHANNEL.1.recv() {
            match nt {
                NotifyType::Status(status) => {
                    (*FOOT_SHOW.lock().unwrap()).set_status(&status);
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
                    RESUTL_VIEW.lock().unwrap().set_text(&*json_handle::pretty_json(&json));
                    CHANNEL.0.clone().send(NotifyType::Result(ComputeResult::Normal));
                    CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Ready));
                }
                NotifyType::SearchTree(pattern) => {
                    let set_color = TREE_VIEW.lock().unwrap().search_nodes(pattern);
                    if set_color {
                        let mut bind = APP_WINDOW.lock().unwrap();
                        let w = bind.get_window();
                        w.redraw();
                    }

                    CHANNEL.0.clone().send(NotifyType::Result(ComputeResult::Normal));
                    CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Ready));
                }
                _ => {}
            }
        }
    }
}

fn listen_on_action() {
    {
        let mut btns = ACTION_BTNS.lock().unwrap();
        let mut parse_btn = &mut btns[0];
        parse_btn.set_callback(|_| {
            let mut bind = APP_WINDOW.lock().unwrap();
            let w = bind.get_window();
            w.redraw(); // this is for Tree view to display tree; without `redraw`, the tree wouldn't show. why?
        });
    }
    {
        let mut btns = ACTION_BTNS.lock().unwrap();
        let mut search_btn = &mut btns[1];
        search_btn.set_callback(|_| {
            let mut bar = TREE_SEARCH_BAR.lock().unwrap().get_bar();
            let mut tree = TREE_MAIN.lock().unwrap();
            let is_show = bar.visible();

            let w = tree.w();
            let h = tree.h();
            if is_show {
                bar.hide();
                tree.set_size(w, h + ACTION_BUTTON_HEIGHT);
                let mut inbox = TREE_SEARCH_BOX.lock().unwrap();
                inbox.set_value("");
            } else {
                tree.set_size(w, h - ACTION_BUTTON_HEIGHT);
                bar.show();
            }
            let mut bind = APP_WINDOW.lock().unwrap();
            let win = bind.get_window();
            win.redraw(); // save as above...why
        })
    }
    {
        let mut btns = ACTION_BTNS.lock().unwrap();
        let mut copy_btn = &mut btns[2];
        copy_btn.set_callback(|_| {
            let mut bind = RESUTL_VIEW.lock().unwrap();
            let buffer = bind.text();
            if buffer.trim().len() == 0 {
                CHANNEL.0.clone().send(NotifyType::Result(ComputeResult::Error(String::from("didn't copy: empty content"))));
                return;
            }
            let cb = ClipboardContext::new();
            if let Err(er) = cb {
                CHANNEL.0.clone().send(NotifyType::Result(ComputeResult::Error(String::from(er.to_string()))));
                return;
            }
            let mut clipboard_context = cb.unwrap();
            let result = clipboard_context.set_contents(buffer);
            if let Err(er) = result {
                CHANNEL.0.clone().send(NotifyType::Result(ComputeResult::Error(String::from(er.to_string()))));
            } else {
                CHANNEL.0.clone().send(NotifyType::Result(ComputeResult::Error(String::from("copied to clipboard"))));
            }
        });
    }
    {
        let mut btns = ACTION_BTNS.lock().unwrap();
        let mut clear_btn = &mut btns[3];
        clear_btn.set_callback(|_| {
            {
                let mut bind = JSON_INPUT_BOX.lock().unwrap();
                bind.buffer().unwrap().set_text("");
            }
            {
                let mut bind = RESUTL_VIEW.lock().unwrap();
                bind.set_text("");
            }
            {
                let bind = TREE_VIEW.lock().unwrap();
                bind.clear()
            }
            {
                let mut search_box = TREE_SEARCH_BOX.lock().unwrap();
                search_box.set_value("");
            }
        });
    }
    {
        let mut do_search_btn = TREE_SEARCH_BTN.lock().unwrap();
        do_search_btn.set_callback(|_| {
            let inbox = TREE_SEARCH_BOX.lock().unwrap();
            let binding = inbox.value();
            let pattern = binding.trim();

            CHANNEL.0.clone().send(NotifyType::Status(ComputeStatus::Computing));
            CHANNEL.0.clone().send(NotifyType::SearchTree(pattern.into()));
        });
    }
}