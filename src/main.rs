use std::rc::Rc;
use component::window::AppWindow;
use data::GLOBAL_CHANNEL;
use fltk::app::{Receiver, Scheme, Sender};
use fltk::app;
use fltk::prelude::{GroupExt, WidgetExt};
use crate::data::{get_receiver, get_sender, set_receiver, set_sender};
use crate::data::notify_enum::NotifyType;

mod component;
mod logic;
mod data;

fn main() {
    let mut app = app::App::default();
    app.set_scheme(Scheme::Plastic);

    let wind = AppWindow::new();
    let mut wind = wind.get_window();
    wind.make_resizable(true);
    wind.show();

    let (s, r) = app::channel::<NotifyType>();
    let (_, mut rb) = GLOBAL_CHANNEL;
    match get_sender() {
        Some(_) => {},
        None => {set_sender(s)},
    }
    match get_receiver() {
        Some(_) => {},
        None => {set_receiver(r)},
    }

    let r = get_receiver();
    match r {
        None => {panic!("12222")}
        Some(_) => {}
    }

    let constr = get_receiver().unwrap().clone();
    while app.wait() {
        if let Some(msg) = constr.recv() {
            match msg {
                NotifyType::Input(s) => println!("{}", s),
                _ => (), // 什么都不做
            }
        }
    }
    
    app.run().unwrap();
}


