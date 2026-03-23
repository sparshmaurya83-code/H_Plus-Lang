use crate::interpreter::Value;

pub fn call(name: &str, args: Vec<Value>) -> Option<Value> {
    match name {
        "len" => {
            if let Value::String(s) = &args[0] {
                Some(Value::Number(s.len() as f64))
            } else {
                None
            }
        }

        "print" => {
            println!("{:?}", args[0]);
            Some(Value::Null)
        }

        _ => None,
    }
}