pub fn call_builtin(module: &str, func: &str, args: Vec<String>) -> Option<String> {
    match module {
        "math" => match func {
            "add" => {
                let a: f64 = args[0].parse().ok()?;
                let b: f64 = args[1].parse().ok()?;
                Some((a + b).to_string())
            }
            "fib" => {
                let n: u32 = args[0].parse().ok()?;
                Some(fib(n).to_string())
            }
            _ => None,
        },

        "text" => match func {
            "upper" => Some(args[0].to_uppercase()),
            "len" => Some(args[0].len().to_string()),
            _ => None,
        },

        "files" => match func {
            "exists" => {
                let path = &args[0];
                Some(std::path::Path::new(path).exists().to_string())
            }
            "read" => {
                let path = &args[0];
                std::fs::read_to_string(path).ok()
            }
            _ => None,
        },

        _ => None,
    }
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}