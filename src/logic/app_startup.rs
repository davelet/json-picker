use std::{fs, thread};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;
use std::cell::RefCell;
use toml_edit::Item::Table as ItemTable;
use toml_edit::{value, DocumentMut, Item, Table};

use crate::data::constants::{SYS_PARAM_IN_THREAD_LOCAL_KEY, SYS_PARAM_LOCATION_H, SYS_PARAM_LOCATION_W, SYS_PARAM_LOCATION_X, SYS_PARAM_LOCATION_Y, SYS_PARAM_SNAPSHOT_JSON, SYSTEM_PARAM_FILE_DIR, SYSTEM_PARAM_FILE_PATH, SYSTEM_PARAM_LOCATION_KEY, SYSTEM_PARAM_SNAPSHOT_KEY};

thread_local! {
    static LOCAL: RefCell<DocumentMut> = RefCell::new(DocumentMut::from_str(&format!("{}=1", SYS_PARAM_IN_THREAD_LOCAL_KEY)).unwrap());
}

/// read from toml
fn input() -> DocumentMut {
    let doc = LOCAL.with(|mut c| c.borrow().clone());
    let init = doc.contains_key(SYS_PARAM_IN_THREAD_LOCAL_KEY);
    if !init {
        return doc;
    }
    let file_path = format!("{}{}", SYSTEM_PARAM_FILE_DIR, SYSTEM_PARAM_FILE_PATH);
    let path = Path::new(&file_path);

    let mut file = match File::open(&path) {
        Err(_) => return DocumentMut::new(),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    let doc = match file.read_to_string(&mut s) {
        Err(_) => DocumentMut::new(),
        Ok(_) => DocumentMut::from_str(&*s).unwrap(),
    };
    LOCAL.with(|c| {
        *c.borrow_mut() = doc
    });
    LOCAL.with(|mut c| c.borrow().clone())
}

/// write to toml file
fn output(toml: DocumentMut) {
    let str = toml.to_string();
    let file_path = format!("{}{}", SYSTEM_PARAM_FILE_DIR, SYSTEM_PARAM_FILE_PATH);
    let wr = fs::write(file_path, str.clone());
    if let Err(_e) = wr {
        let _ = create_dir_all(SYSTEM_PARAM_FILE_DIR);
        let f = File::create(SYSTEM_PARAM_FILE_PATH);
        if let Ok(mut f) = f {
            let _r = f.write_all(str.as_bytes());
        }
    }
}

fn get_table_block(saved: &DocumentMut, key: &str) -> Table {
    let option = saved.get(key);
    let location = match option {
        None => Table::new(),
        Some(item) => {
            if let Item::Table(t) = item {
                t.clone()
            } else {
                Table::new()
            }
        }
    };
    location
}

pub(crate) fn store_location(x: i64, y: i64, w: i64, h: i64) {
    let mut saved = input();
    let mut location = get_table_block(&saved, SYSTEM_PARAM_LOCATION_KEY);

    location[SYS_PARAM_LOCATION_X] = value(x);
    location[SYS_PARAM_LOCATION_Y] = value(y);
    location[SYS_PARAM_LOCATION_W] = value(w);
    location[SYS_PARAM_LOCATION_H] = value(h);
    saved.insert(SYSTEM_PARAM_LOCATION_KEY, ItemTable(location));
    output(saved);
}

pub(crate) fn load_location() -> Option<(i64, i64, i64, i64)> {
    let config = input();
    let block = config.get(SYSTEM_PARAM_LOCATION_KEY);
    match block {
        None => None,
        Some(item) => {
            if let ItemTable(t) = item {
                let x = t[SYS_PARAM_LOCATION_X].as_integer().unwrap();
                let y = t[SYS_PARAM_LOCATION_Y].as_integer().unwrap();
                let w = t[SYS_PARAM_LOCATION_W].as_integer().unwrap();
                let h = t[SYS_PARAM_LOCATION_H].as_integer().unwrap();
                Some((x, y, w, h))
            } else {
                None
            }
        }
    }
}

pub(crate) fn store_snapshot(json: &String) {
    let mut saved = input();
    let mut snapshot = get_table_block(&saved, SYSTEM_PARAM_SNAPSHOT_KEY);
    snapshot[SYS_PARAM_SNAPSHOT_JSON] = value(json);
    saved.insert(SYSTEM_PARAM_SNAPSHOT_KEY, ItemTable(snapshot));
    output(saved)
}

pub(crate) fn load_snapshot() -> Option<String> {
    let config = input();
    let block = config.get(SYSTEM_PARAM_SNAPSHOT_KEY);
    match block {
        Some(x) => {
            if let ItemTable(t) = x {
                Some(t[SYS_PARAM_SNAPSHOT_JSON].as_str().unwrap().into())
            } else {
                None
            }
        }
        None => None,
    }
}
