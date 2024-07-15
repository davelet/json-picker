use std::fs;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

use toml_edit::{DocumentMut, value};

use crate::data::constants::{APP_PARAM_FILE_DIR, APP_SETTING_FILE_PATH, JSON_SIZE_LIMIT, SYS_SETTINGS_LENGTH_LIMIT, SYSTEM_PARAM_FILE_PATH};
use crate::data::singleton::HOME_DIR;

/// read from toml
fn input() -> Result<DocumentMut, String> {
    let mut guard = HOME_DIR.lock().unwrap();
    let home_dir = guard.get_mut();
    if home_dir.len() == 0 {
        return Err("home dir access error".into());
    }

    let file_path = format!("{}{}{}", home_dir, APP_PARAM_FILE_DIR, APP_SETTING_FILE_PATH);
    let path = Path::new(&file_path);

    let mut file = match File::open(&path) {
        Err(_) => return Ok(DocumentMut::new()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    let doc = match file.read_to_string(&mut s) {
        Err(_) => DocumentMut::new(),
        Ok(_) => DocumentMut::from_str(&*s).unwrap(),
    };
    Ok(doc)
}

fn output(toml: DocumentMut) {
    let str = toml.to_string();
    let file_path = format!("{}{}{}", HOME_DIR.lock().unwrap().get_mut(), APP_PARAM_FILE_DIR, APP_SETTING_FILE_PATH);
    let wr = fs::write(file_path, str.clone());
    if let Err(_) = wr {
        let _ = create_dir_all(format!("{}{}", HOME_DIR.lock().unwrap().get_mut(), APP_PARAM_FILE_DIR));
        let f = File::create(APP_SETTING_FILE_PATH);
        if let Ok(mut f) = f {
            let _r = f.write_all(str.as_bytes());
        }
    }
}

pub(crate) fn get_limit() -> i64 {
    let doc = input();
    if let Ok(doc) = doc {
        let limit = doc.get(SYS_SETTINGS_LENGTH_LIMIT);
        if let Some(limit) = limit {
            let option = limit.as_integer();
            if let Some(limit) = option {
                return limit;
            }
        }
    }
    JSON_SIZE_LIMIT as i64
}

pub(crate) fn set_limit(limit: i64) {
    let doc = input();
    if let Ok(mut doc) = doc {
        doc[SYS_SETTINGS_LENGTH_LIMIT] = value(limit);
        output(doc)
    }
}