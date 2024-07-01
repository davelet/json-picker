use fltk::tree::{Tree, TreeItem};
use serde_json::Value;

use crate::data::stack::Stack;

pub(crate) fn pretty_json(json: &Value) -> String {
    pretty_json_with_indent(json, 0)
}

fn pretty_json_with_indent(json: &Value, indent: usize) -> String {
    let mut output = String::new();
    match json {
        Value::Object(obj) => {
            output += "{\n";
            for (key, value) in obj {
                let spaces = " ".repeat(indent * 2);
                output += &format!(
                    "{}  \"{}\": {},\n",
                    spaces,
                    key,
                    pretty_json_with_indent(value, indent + 1)
                );
            }
            trim_json(&mut output); // 移除最后的逗号
            output += &format!("\n{}}}", " ".repeat(indent));
        }
        Value::Array(arr) => {
            output += "[\n";
            for value in arr {
                let spaces = " ".repeat(indent * 2);
                output += &format!(
                    "{}  {},\n",
                    spaces,
                    pretty_json_with_indent(value, indent + 1)
                );
            }
            trim_json(&mut output); // 移除最后的逗号
            output += &format!("\n{}]", " ".repeat(indent));
        }
        Value::String(s) => {
            output += &format!("\"{}\"", s.replace("\"", "\\\"")); // 转义双引号
        }
        Value::Bool(b) => {
            output += if *b { "true" } else { "false" };
        }
        Value::Number(n) => {
            output += &n.to_string();
        }
        Value::Null => {
            output += "null";
        }
    }
    output
}

fn trim_json(line: &mut String) {
    if line.ends_with('\n') {
        line.pop();
        trim_json(line);
    } else if line.ends_with(',') {
        line.pop();
    }
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
