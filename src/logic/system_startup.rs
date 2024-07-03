use toml_edit::{DocumentMut, value};

pub(crate) fn store_location(x: i64, y: i64, w: i64, h: i64) {
    let mut location = DocumentMut::new();
    location["x"] = value(x);
    location["y"] = value(y);
    location["w"] = value(w);
    location["h"] = value(h);
    println!("location {x} {y} {w} {h}")
}