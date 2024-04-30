pub(crate) mod notify_enum;

use std::cell::Cell;
use std::rc::Rc;

use fltk::app::{Receiver, Sender};
use crate::data::notify_enum::NotifyType;

pub(crate) const DEFAULT_WIDTH: i32 = 800;
pub(crate) const DEFAULT_HEIGHT: i32 = 600;
pub(crate) const MIN_WIDTH: i32 = 800;
pub(crate) const MIN_HEIGHT: i32 = 600;

pub(crate) const COLUMN_COUNT: i32 = 3;

pub(crate) const GLOBAL_CHANNEL: (Cell<Option<Rc<Sender<NotifyType>>>>, Cell<Option<Rc<Receiver<NotifyType>>>>) = (Cell::new(None), Cell::new(None));

pub(crate) fn get_sender() -> Option<Rc<Sender<NotifyType>>> {
    // let (mut sb,_) = GLOBAL_CHANNEL;
    // let s1 = sb.get_mut();
    // s1.clone();
    GLOBAL_CHANNEL.0.get_mut().clone()
}

pub(crate) fn set_sender(s: Sender<NotifyType>) {
    // let mut sb = GLOBAL_CHANNEL.0;
    GLOBAL_CHANNEL.0.set(Some(Rc::new(s)))
}

pub(crate) fn get_receiver() -> Option<Rc<Receiver<NotifyType>>> {
    // let (_, mut rb) = GLOBAL_CHANNEL;
    // let r = rb.get_mut();
    // r.clone();
    GLOBAL_CHANNEL.1.get_mut().clone()
}

pub(crate) fn set_receiver(s: Receiver<NotifyType>) {
    // let mut sb = GLOBAL_CHANNEL.1;
    GLOBAL_CHANNEL.1.set(Some(Rc::new(s)))
}