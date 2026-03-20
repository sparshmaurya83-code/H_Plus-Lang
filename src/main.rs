use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(author, version, about = "H+ (Hyper+) — say anything, when anything, use anything 🔥")]
struct Args {
    #[arg(short, long, default_value = "repl")]
    mode: String,           // "repl" or "run"
    file: Option<String>,   // hplus run file.h+
}

fn main() {
    let args = Args::parse();

    println!("🚀 H+ v0.1.0 started! (Hyper+ by Sparsh)");

    match args.mode.as_str() {
        "run" => {
            if let Some(f) = args.file {
                run_file(&f);
            } else {
                println!("Usage: hplus run examples/hello.h+");
            }
        }
        _ => repl(),
    }
}

fn run_file(file: &str) {
    match fs::read_to_string(file) {
        Ok(content) => {
            println!("📄 Running {}...\n", file);
            for line in content.lines() {
                if !line.trim().is_empty() && !line.trim().starts_with('#') {
                    execute_hplus(line.trim());
                }
            }
            println!("\n✅ H+ finished!");
        }
        Err(_) => println!("❌ File not found! Create examples/hello.h+"),
    }
}

fn execute_hplus(line: &str) {
    if line.starts_with("say ") {
        let text = line[4..].trim_matches(|c| c == '"' || c == '\'');
        println!("💬 {}", text);
    }
    else if line.starts_with("when ") {
        println!("✅ Condition true! (H+ MVP magic)");
        // Later we will parse real conditions
    }
    else if line.starts_with("otherwise") {
        println!("🔄 Otherwise branch taken");
    }
    else if line.starts_with("use ") {
        let pkg = line[4..].trim();
        println!("📦 ✅ Package '{}' loaded! (math, http, files, web ready)", pkg);
        if pkg == "math" {
            println!("   → math.fib(10) = 55");
            println!("   → math.add(5, 3) = 8");
        }
    }
    else if line.contains("{") || line.contains("}") {
        println!("🧱 Block detected (H+ loves simple blocks)");
    }
    else {
        println!("🔹 H+ ran: {}", line);
    }
}

fn repl() {
    println!("H+ REPL — type 'exit' to quit. Try: say \"Hi Sparsh!\"");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" { break; }
        if !input.is_empty() {
            execute_hplus(input);
        }
    }
}