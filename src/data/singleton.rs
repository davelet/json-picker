use std::cell::Cell;
use std::sync::Mutex;

use fltk::button::Button;
use fltk::input::Input;
use fltk::prelude::WidgetExt;
use fltk::text::{TextBuffer, TextEditor};
use fltk::tree::Tree;
use serde_json::Value;
use serde_json::Value::Null;

use crate::component::border_panel::WholeViewPanel;
use crate::component::labeled_line::LabeledLine;
use crate::component::search_bar::SearchBar;
use crate::component::structure_tree::JsonStructure;
use crate::component::window::AppWindow;
use crate::data::constants::{ACTION_BUTTON_COUNT, ACTION_BUTTON_HEIGHT, ACTION_BUTTON_LABELS, COLUMN_COUNT, CONTENT_HEIGHT, DEFAULT_HEIGHT, DEFAULT_WIDTH, SEARCH_BAR_HEIGHT, SEARCH_BTN_LABEL, SEARCH_BTN_WIDTH};
use crate::data::notify_enum::NotifyType;
use crate::logic::workers::startup_tasks::{AppWindowLocationLoadTask, StartupTask};
use crate::logic::workers::ui_tasks::{AppWindowLocationPersistenceTask, ComputeOnSelectedTask, HaltWaitingStatusTask, ParsedJsonStringPersistenceTask, UiTask};

lazy_static::lazy_static! {
    pub(crate) static ref APP_WINDOW: Mutex<AppWindow> = Mutex::new(AppWindow::new());
    pub(crate) static ref WHOLE_VIEW: Mutex<WholeViewPanel> = Mutex::new(WholeViewPanel::new(DEFAULT_WIDTH, DEFAULT_HEIGHT));
    pub(crate) static ref FOOT_SHOW: Mutex<LabeledLine> = Mutex::new(LabeledLine::init_footer(DEFAULT_WIDTH));
    pub(crate) static ref RESUTL_VIEW: Mutex<TextBuffer> = Mutex::new(TextBuffer::default());
    pub(crate) static ref TREE_VIEW: Mutex<JsonStructure> = Mutex::new(JsonStructure::new(DEFAULT_WIDTH / COLUMN_COUNT, CONTENT_HEIGHT));
    pub(crate) static ref TREE_MAIN: Mutex<Tree> = Mutex::new(Tree::default().with_size(DEFAULT_WIDTH / COLUMN_COUNT, CONTENT_HEIGHT));
    pub(crate) static ref JSON_INPUT_BOX: Mutex<TextEditor> = Mutex::new(TextEditor::default().with_size(DEFAULT_WIDTH / COLUMN_COUNT, CONTENT_HEIGHT));
    pub(crate) static ref ACTION_BTNS: Mutex<[Button; ACTION_BUTTON_COUNT as usize]> = Mutex::new([
            Button::default().with_size(DEFAULT_WIDTH/ ACTION_BUTTON_COUNT, ACTION_BUTTON_HEIGHT).with_label(ACTION_BUTTON_LABELS[0]),
            Button::default().with_size(DEFAULT_WIDTH/ ACTION_BUTTON_COUNT, ACTION_BUTTON_HEIGHT).with_label(ACTION_BUTTON_LABELS[1]),
            Button::default().with_size(DEFAULT_WIDTH/ ACTION_BUTTON_COUNT, ACTION_BUTTON_HEIGHT).with_label(ACTION_BUTTON_LABELS[2]),
            Button::default().with_size(DEFAULT_WIDTH/ ACTION_BUTTON_COUNT, ACTION_BUTTON_HEIGHT).with_label(ACTION_BUTTON_LABELS[3])
    ]);
    pub(crate) static ref TREE_SEARCH_BOX: Mutex<Input> = Mutex::new(Input::default().with_size(DEFAULT_WIDTH / COLUMN_COUNT - SEARCH_BTN_WIDTH, SEARCH_BAR_HEIGHT));
    pub(crate) static ref TREE_SEARCH_BTN: Mutex<Button> = Mutex::new(Button::default().with_size(SEARCH_BTN_WIDTH, SEARCH_BAR_HEIGHT).with_label(SEARCH_BTN_LABEL));
    pub(crate) static ref TREE_SEARCH_BAR: Mutex<SearchBar> = Mutex::new(SearchBar::new(DEFAULT_WIDTH / COLUMN_COUNT));

    pub(crate) static ref CHANNEL: (fltk::app::Sender<NotifyType>, fltk::app::Receiver<NotifyType>) = fltk::app::channel();
    pub(crate) static ref STATUS_TASK: Mutex<HaltWaitingStatusTask> = Mutex::new(HaltWaitingStatusTask::new());
    pub(crate) static ref COMPUTE_TASK: Mutex<ComputeOnSelectedTask> = Mutex::new(ComputeOnSelectedTask::new());
    pub(crate) static ref LOCATION_TASK: Mutex<AppWindowLocationPersistenceTask> = Mutex::new(AppWindowLocationPersistenceTask::new());
    pub(crate) static ref LOAD_LOCATION_TASK: Mutex<AppWindowLocationLoadTask> = Mutex::new(AppWindowLocationLoadTask::new());
    pub(crate) static ref JSON_SAVE_TASK: Mutex<ParsedJsonStringPersistenceTask> = Mutex::new(ParsedJsonStringPersistenceTask::new());
    pub(crate) static ref GLOBAL_JSON: Mutex<Cell<Value>> = Mutex::new(Cell::new(Null));

}
