pub trait CustomizedAction {
    fn on_parent_resize(&self, width: i32, height: i32);
}

pub(super) trait CustomizedComponent: CustomizedAction {

}
