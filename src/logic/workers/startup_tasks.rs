use fltk::prelude::WidgetExt;
use crate::data::singleton::APP_WINDOW;
use crate::data::task_bo::AppWindowLocationTaskParam;
use crate::logic::system_startup::load_location;
use crate::logic::workers::ui_tasks::UiTask;

pub(crate) trait StartupTask<D> {
    fn new() -> Self;
    fn before_execute(&mut self, data: D) -> bool;
    fn execute(&mut self, data: D);
}

pub(crate) struct AppWindowLocationLoadTask {
    location: Option<AppWindowLocationTaskParam>,
}

impl StartupTask<bool> for AppWindowLocationLoadTask {
    fn new() -> Self {
        Self { location: None }
    }

    fn before_execute(&mut self, _data: bool) -> bool {
        let saved = load_location();
        if let Some((x, y, w, h)) = saved {
            self.location = Some(AppWindowLocationTaskParam::new(x, y, w, h));
            return true;
        }
        false
    }

    fn execute(&mut self, _data: bool) {
        let mut window = APP_WINDOW.lock().unwrap();
        let wind = window.get_window();
        let data = self.location.as_ref().unwrap();
        wind.resize(data.x() as i32, data.y() as i32, data.w() as i32, data.h() as i32);
    }

}



