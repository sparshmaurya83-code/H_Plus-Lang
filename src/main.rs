use clap::{Parser, Subcommand};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(name = "hplus")]
#[command(
    about = "H+ (Hyper+) — say, when, use. World's simplest language! 🔥",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a .h+ file
    Run {
        /// The file to execute
        file: String,
    },
    /// Start interactive REPL
    Repl,
    /// List built-in packages
    Packages,
}

#[derive(Debug, Clone)]
struct Package {
    name: &'static str,
    description: &'static str,
    functions: &'static [&'static str],
}

#[derive(Debug, Default)]
struct Runtime {
    loaded_packages: HashSet<String>,
    variables: HashMap<String, String>,
}

fn main() {
    let cli = Cli::parse();

    println!("🚀 H+ v0.2.0 started! (Hyper+ by Sparsh)");

    match cli.command.unwrap_or(Commands::Repl) {
        Commands::Run { file } => run_file(&file),
        Commands::Repl => repl(),
        Commands::Packages => list_packages(),
    }
}

fn built_in_packages() -> HashMap<&'static str, Package> {
    HashMap::from([
        (
            "math",
            Package {
                name: "math",
                description: "Fast math helpers for tiny programs",
                functions: &["math.add(a, b)", "math.sub(a, b)", "math.fib(n)"],
            },
        ),
        (
            "text",
            Package {
                name: "text",
                description: "Simple text tools",
                functions: &["text.upper(value)", "text.lower(value)", "text.len(value)"],
            },
        ),
        (
            "files",
            Package {
                name: "files",
                description: "Minimal file helpers",
                functions: &["files.read(path)", "files.exists(path)"],
            },
        ),
        (
            "web",
            Package {
                name: "web",
                description: "Tiny web request helpers",
                functions: &["web.get(url)", "web.status(url)"],
            },
        ),
    ])
}

fn list_packages() {
    println!("📦 Built-in H+ packages:\n");
    for package in built_in_packages().into_values() {
        println!("• {} — {}", package.name, package.description);
        for function in package.functions {
            println!("    {}", function);
        }
    }
}

fn run_file(file: &str) {
    match fs::read_to_string(file) {
        Ok(content) => {
            let mut runtime = Runtime::default();
            println!("📄 Running {}...\n", file);

            for (index, line) in content.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }

                if let Err(error) = execute_hplus(trimmed, &mut runtime) {
                    println!("❌ Line {}: {}", index + 1, error);
                    return;
                }
            }

            println!("\n✅ H+ finished!");
        }
        Err(_) => println!("❌ File not found! Create examples/hello.h+"),
    }
}

fn execute_hplus(line: &str, runtime: &mut Runtime) -> Result<(), String> {
    if line.starts_with("use ") {
        let pkg = line[4..].trim();
        return load_package(pkg, runtime);
    }

    if line.starts_with("let ") {
        return assign_variable(line, runtime);
    }

    if line.starts_with("say ") {
        let output = evaluate_expression(&line[4..], runtime)?;
        println!("💬 {}", output);
        return Ok(());
    }

    if line.starts_with("when ") {
        println!("✅ Condition true! (H+ MVP magic)");
        return Ok(());
    }

    if line.starts_with("otherwise") {
        println!("🔄 Otherwise branch taken");
        return Ok(());
    }

    if line.contains('{') || line.contains('}') {
        println!("🧱 Block detected (H+ loves simple blocks)");
        return Ok(());
    }

    let output = evaluate_expression(line, runtime)?;
    println!("🔹 H+ ran: {}", output);
    Ok(())
}

fn load_package(package_name: &str, runtime: &mut Runtime) -> Result<(), String> {
    let packages = built_in_packages();
    let Some(package) = packages.get(package_name) else {
        let available = packages
            .keys()
            .copied()
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "Unknown package '{}'. Available packages: {}",
            package_name, available
        ));
    };

    if runtime.loaded_packages.insert(package_name.to_string()) {
        println!("📦 ✅ Package '{}' loaded!", package.name);
        println!("   → {}", package.description);
    } else {
        println!("📦 ℹ️ Package '{}' already loaded", package.name);
    }

    Ok(())
}

fn assign_variable(line: &str, runtime: &mut Runtime) -> Result<(), String> {
    let Some((name, value)) = line[4..].split_once('=') else {
        return Err("Invalid variable syntax. Use: let name = value".to_string());
    };

    let key = name.trim();
    if key.is_empty() {
        return Err("Variable name cannot be empty".to_string());
    }

    let evaluated = evaluate_expression(value.trim(), runtime)?;
    runtime
        .variables
        .insert(key.to_string(), evaluated.clone());

    println!("📝 {} = {}", key, evaluated);
    Ok(())
}

fn evaluate_expression(expr: &str, runtime: &Runtime) -> Result<String, String> {
    let trimmed = expr.trim();

    if trimmed.is_empty() {
        return Ok(String::new());
    }

    if is_quoted(trimmed) {
        return Ok(unquote(trimmed));
    }

    if let Some(value) = runtime.variables.get(trimmed) {
        return Ok(value.clone());
    }

    if trimmed.contains('(') && trimmed.ends_with(')') && trimmed.contains('.') {
        return evaluate_package_call(trimmed, runtime);
    }

    Ok(trimmed.to_string())
}

fn evaluate_package_call(call: &str, runtime: &Runtime) -> Result<String, String> {
    let Some((package_name, rest)) = call.split_once('.') else {
        return Err(format!("Invalid package call '{}'", call));
    };

    if !runtime.loaded_packages.contains(package_name) {
        return Err(format!(
            "Package '{}' is not loaded. Add: use {}",
            package_name, package_name
        ));
    }

    let Some(open_paren) = rest.find('(') else {
        return Err(format!("Invalid package function call '{}'", call));
    };

    let function_name = rest[..open_paren].trim();
    let args_section = &rest[open_paren + 1..rest.len() - 1];
    let args = parse_arguments(args_section, runtime)?;

    match (package_name, function_name) {
        ("math", "add") => numeric_binary(&args, |a, b| a + b),
        ("math", "sub") => numeric_binary(&args, |a, b| a - b),
        ("math", "fib") => {
            let n = parse_single_u64(&args, "math.fib")?;
            Ok(fibonacci(n).to_string())
        }
        ("text", "upper") => string_single(&args, |value| value.to_uppercase(), "text.upper"),
        ("text", "lower") => string_single(&args, |value| value.to_lowercase(), "text.lower"),
        ("text", "len") => string_single(&args, |value| value.len().to_string(), "text.len"),
        ("files", "read") => {
            let path = parse_single_string(&args, "files.read")?;
            fs::read_to_string(&path)
                .map(|content| content.replace('\n', "\\n"))
                .map_err(|_| format!("Cannot read file '{}'", path))
        }
        ("files", "exists") => {
            let path = parse_single_string(&args, "files.exists")?;
            Ok(fs::metadata(path).is_ok().to_string())
        }
        ("web", "get") => {
            let url = parse_single_string(&args, "web.get")?;
            Ok(format!("fake-response-from:{}", url))
        }
        ("web", "status") => {
            let _url = parse_single_string(&args, "web.status")?;
            Ok("200".to_string())
        }
        _ => Err(format!(
            "Unknown function '{}.{}'",
            package_name, function_name
        )),
    }
}

fn parse_arguments(args: &str, runtime: &Runtime) -> Result<Vec<String>, String> {
    if args.trim().is_empty() {
        return Ok(vec![]);
    }

    args.split(',')
        .map(|part| evaluate_expression(part.trim(), runtime))
        .collect()
}

fn numeric_binary(args: &[String], op: impl Fn(i64, i64) -> i64) -> Result<String, String> {
    if args.len() != 2 {
        return Err("This function expects exactly 2 numbers".to_string());
    }

    let left = args[0]
        .parse::<i64>()
        .map_err(|_| format!("'{}' is not a number", args[0]))?;
    let right = args[1]
        .parse::<i64>()
        .map_err(|_| format!("'{}' is not a number", args[1]))?;

    Ok(op(left, right).to_string())
}

fn parse_single_u64(args: &[String], name: &str) -> Result<u64, String> {
    if args.len() != 1 {
        return Err(format!("{} expects exactly 1 number", name));
    }

    args[0]
        .parse::<u64>()
        .map_err(|_| format!("'{}' is not a valid number", args[0]))
}

fn parse_single_string(args: &[String], name: &str) -> Result<String, String> {
    if args.len() != 1 {
        return Err(format!("{} expects exactly 1 value", name));
    }

    Ok(args[0].clone())
}

fn string_single(
    args: &[String],
    op: impl Fn(&str) -> String,
    name: &str,
) -> Result<String, String> {
    if args.len() != 1 {
        return Err(format!("{} expects exactly 1 value", name));
    }

    Ok(op(&args[0]))
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut previous = 0;
            let mut current = 1;

            for _ in 2..=n {
                let next = previous + current;
                previous = current;
                current = next;
            }

            current
        }
    }
}

fn is_quoted(value: &str) -> bool {
    (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''))
}

fn unquote(value: &str) -> String {
    value[1..value.len() - 1].to_string()
}

fn repl() {
    println!("H+ REPL — type 'exit' to quit. Try: use math");
    let mut runtime = Runtime::default();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let input = match line {
            Ok(s) => s.trim().to_string(),
            Err(_) => break,
        };

        if input == "exit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        if let Err(error) = execute_hplus(&input, &mut runtime) {
            println!("❌ {}", error);
        }
    }
}
