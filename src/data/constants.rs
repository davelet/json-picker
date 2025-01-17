pub(crate) const APP_NAME: &str = "JSON PICKER";
pub(crate) const DEFAULT_WIDTH: i32 = 900;
pub(crate) const DEFAULT_HEIGHT: i32 = 600;
pub(crate) const MIN_WIDTH: i32 = 800;
pub(crate) const MIN_HEIGHT: i32 = 600;
pub(crate) const LOADING_WIDTH: i32 = 600;
pub(crate) const LOADING_HEIGHT: i32 = 400;
pub(crate) const HEADER_HEIGHT: i32 = 20;
pub(crate) const SEARCH_BAR_HEIGHT: i32 = 30;
pub(crate) const SEARCH_BTN_WIDTH: i32 = 60;
pub(crate) const SEARCH_BTN_LABEL: &str = "search";
pub(crate) const FOOTER_HEIGHT: i32 = 20;
pub(crate) const COLUMN_COUNT: i32 = 3;
pub(crate) const ACTION_BUTTON_COUNT: i32 = 4;
pub(crate) const ACTION_BUTTON_HEIGHT: i32 = 30;
pub(crate) const ACTION_BUTTON_LABELS: [&str; 4] = ["Parse Input", "Show/Hide Search Bar", "Copy Result", "Clear Input and Result"];
pub(crate) const CONTENT_HEIGHT: i32 = DEFAULT_HEIGHT - HEADER_HEIGHT - FOOTER_HEIGHT - ACTION_BUTTON_HEIGHT;

pub(crate) const START_TIMEOUT: f64 = 0.5;
pub(crate) const JSON_SIZE_LIMIT: usize = 10_000_000;
pub(crate) const JSON_SIZE_WARN: &str = "input too long";
pub(crate) const TREE_LABEL_SPLITTER: &str = ":";
pub(crate) const APP_PARAM_FILE_DIR: &str = "/.json_picker";
pub(crate) const SYSTEM_PARAM_FILE_PATH: &str = "/params.toml";
pub(crate) const APP_SETTING_FILE_PATH: &str = "/settings.toml";
pub(crate) const SYSTEM_PARAM_LOCATION_KEY: &str = "location";
pub(crate) const SYSTEM_PARAM_SNAPSHOT_KEY: &str = "snapshot";
pub(crate) const SYS_PARAM_IN_THREAD_LOCAL_KEY: &str = "cache_init";
pub(crate) const SYS_PARAM_LOCATION_X: &str = "x";
pub(crate) const SYS_PARAM_LOCATION_Y: &str = "y";
pub(crate) const SYS_PARAM_LOCATION_W: &str = "w";
pub(crate) const SYS_PARAM_LOCATION_H: &str = "h";
pub(crate) const SYS_PARAM_SNAPSHOT_JSON: &str = "j";
pub(crate) const SYS_SETTINGS_LENGTH_LIMIT: &str = "limit";
pub(crate) const SYS_SETTINGS_LINE_HEIGHT: i32 = 20;
pub(crate) const SYS_SETTINGS_WINDOW_WIDTH: i32 = 300;
pub(crate) const SYS_SETTINGS_INPUT_HEIGHT: i32 = 30;
