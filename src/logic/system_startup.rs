use std::fs;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

use toml_edit::{DocumentMut, Item, Table, value};
use toml_edit::Item::Table as ItemTable;

use crate::data::constants::{SYSTEM_PARAM_FILE_DIR, SYSTEM_PARAM_FILE_PATH, SYSTEM_PARAM_LOCATION_KEY};

/// read from toml
fn input() -> DocumentMut {
    let file_path = format!("{}{}", SYSTEM_PARAM_FILE_DIR, SYSTEM_PARAM_FILE_PATH);
    let path = Path::new(&file_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => { return DocumentMut::new() }
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => { return DocumentMut::from_str(&*s).unwrap() }
    }
}

/// write to toml file
fn output(toml: DocumentMut) {
    let str = toml.to_string();
    let file_path = format!("{}{}", SYSTEM_PARAM_FILE_DIR, SYSTEM_PARAM_FILE_PATH);
    let wr = fs::write(file_path, str.clone());
    if let Err(e) = wr {
        let _ = create_dir_all(SYSTEM_PARAM_FILE_DIR);
        let f = File::create(SYSTEM_PARAM_FILE_PATH);
        if let Ok(mut f) = f {
            let r = f.write_all(str.as_bytes());
            if let Err(e) = r {
                println!("{}", e.to_string()) // todo ignore?
            }
        }
    }
}

pub(crate) fn store_location(x: i64, y: i64, w: i64, h: i64) {
    let mut saved = input();
    println!("toml {}", saved);
    let option = saved.get(SYSTEM_PARAM_LOCATION_KEY);
    let mut location = match option {
        None => { Table::new() }
        Some(item) => {
            if let Item::Table(t) = item {
                t.clone()
            } else {
                Table::new()
            }
        }
    };
    location["x"] = value(x);
    location["y"] = value(y);
    location["w"] = value(w);
    location["h"] = value(h);
    saved.insert(SYSTEM_PARAM_LOCATION_KEY, ItemTable(location));
    output(saved);
}