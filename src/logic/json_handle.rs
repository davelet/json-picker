use fltk::tree::Tree;
use serde_json::Value;

pub fn pretty_json(json: &Value) -> String {
    pretty_json_with_indent(json, 0)
}
pub fn pretty_json_with_indent(json: &Value, indent: usize) -> String {
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
            output.pop(); // 移除最后的逗号
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
            output.pop(); // 移除最后的逗号
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

pub(crate) fn add_tree_items(tree: &mut Tree, json: &Value, path: String) {
    match json {
        Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Null => {
            tree.add(&*format!("{path}{}", get_enum_name(json)));
        }
        Value::Array(arr) => {
            tree.add(&*format!("{path}Array["));
            for (i, j) in arr.iter().enumerate() {
                add_tree_items(tree, j, format!("{path}Array[/{}: ", i));
            }
            tree.add(&*format!("{path}]"));
        }
        Value::Object(map) => {
            tree.add(&*format!("{path}Object{{"));
            for (ele, v) in map {
                match v {
                    Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Null => {
                        tree.add(&*format!("{path}Object{{/{ele}: {}", get_enum_name(v)));
                    }
                    Value::Array(_) | Value::Object(_) => {
                        add_tree_items(tree, v, format!("{path}Object{{/{ele}: "));
                    }
                };
            }
            tree.add(&*format!("{path}}}"));
        }
    }
}

fn get_enum_name(json: &Value) -> &str {
    match json {
        Value::Null => "Null",
        Value::Bool(_) => "Boolean",
        Value::Number(_) => "Number",
        Value::String(_) => "String",
        Value::Array(_) => "Array",
        Value::Object(_) => "Object",
    }
}
