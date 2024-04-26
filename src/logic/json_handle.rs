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
                output += &format!("{}  \"{}\": {},\n", spaces, key, pretty_json_with_indent(value, indent + 1));
            }
            output.pop(); // 移除最后的逗号
            output += &format!("\n{}}}", " ".repeat(indent));
        }
        Value::Array(arr) => {
            output += "[\n";
            for value in arr {
                let spaces = " ".repeat(indent * 2);
                output += &format!("{}  {},\n", spaces, pretty_json_with_indent(value, indent + 1));
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
