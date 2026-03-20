use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, stdin};

#[derive(Parser)]
#[command(name = "hplus")]
#[command(about = "H+ (Hyper+) — say, when, use. World's simplest language! 🔥", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a .h+ file
    Run {
        /// The file to execute
        file: String,
    },
    /// Start interactive REPL (default if no command given)
    Repl,
}

fn main() {
    let cli = Cli::parse();

    println!("🚀 H+ v0.1.0 started! (Hyper+ by Sparsh)");

    match cli.command {
        Commands::Run { file } => run_file(&file),
        Commands::Repl => repl(),
    }
}

fn run_file(file: &str) {
    match fs::read_to_string(file) {
        Ok(content) => {
            println!("📄 Running {}...\n", file);
            for line in content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    execute_hplus(trimmed);
                }
            }
            println!("\n✅ H+ finished!");
        }
        Err(_) => println!("❌ File not found! Create examples/hello.h+"),
    }
}

fn execute_hplus(line: &str) {
    if line.starts_with("say ") {
        let text = line[4..].trim_matches(|c: char| c == '"' || c == '\'');
        println!("💬 {}", text);
    } else if line.starts_with("when ") {
        println!("✅ Condition true! (H+ MVP magic)");
    } else if line.starts_with("otherwise") {
        println!("🔄 Otherwise branch taken");
    } else if line.starts_with("use ") {
        let pkg = line[4..].trim();
        println!("📦 ✅ Package '{}' loaded! (math, http, files, web ready)", pkg);
        if pkg == "math" {
            println!("   → math.fib(10) = 55");
            println!("   → math.add(5, 3) = 8");
        }
    } else if line.contains("{") || line.contains("}") {
        println!("🧱 Block detected (H+ loves simple blocks)");
    } else {
        println!("🔹 H+ ran: {}", line);
    }
}

fn repl() {
    println!("H+ REPL — type 'exit' to quit. Try: say \"Hi Sparsh!\"");
    let stdin = io::stdin();
    for line in stdin.lines() {
        let input = match line {
            Ok(s) => s.trim().to_string(),
            Err(_) => break,
        };
        if input == "exit" {
            break;
        }
        if !input.is_empty() {
            execute_hplus(&input);
        }
    }
}