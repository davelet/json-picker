use fltk::tree::{Tree, TreeItem};
use serde_json::{to_string_pretty, Value};

use crate::data::stack::Stack;

pub(crate) fn pretty_json(json: &Value) -> String {
    to_string_pretty(json).unwrap()
}

pub(crate) fn add_tree_items(tree: &mut Tree, json: &Value, path: String) {
    let sub_path = format!("{path}{}", get_enum_name(json));
    tree.add(&*sub_path);
    match json {
        Value::Array(arr) => {
            for (i, j) in arr.iter().enumerate() {
                add_tree_items(tree, j, format!("{sub_path}/{}: ", i));
            }
            tree.add(&*format!("{path}]"));
        }
        Value::Object(map) => {
            for (ele, v) in map {
                match v {
                    Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Null => {
                        tree.add(&*format!("{sub_path}/{ele}: {}", get_enum_name(v)));
                    }
                    Value::Array(_) | Value::Object(_) => {
                        add_tree_items(tree, v, format!("{sub_path}/{ele}: "));
                    }
                };
            }
            tree.add(&*format!("{path}}}"));
        }
        _ => {}
    }
}

fn get_enum_name(json: &Value) -> &str {
    match json {
        Value::Null => "Null",
        Value::Bool(_) => "Boolean",
        Value::Number(_) => "Number",
        Value::String(_) => "String",
        Value::Array(_) => "[Array",
        Value::Object(_) => "{Object",
    }
}

pub(crate) fn parse_path_chain(item: &TreeItem) -> Stack<String> {
    let mut stack = Stack::new();
    if item.is_root() {
        return stack;
    }

    let mut temp = Some(item);
    let mut next;
    while let Some(node) = temp {
        if node.is_root() {
            break;
        }
        if let Some(label) = node.label() {
            stack.push(label);
        }
        next = node.parent().unwrap();
        temp = Some(&next);
    }

    return stack;
}
