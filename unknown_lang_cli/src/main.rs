use clap::Parser;
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to compile. If no file is provided, enter unknown-lang REPL.
    #[arg(short, long, value_name = "FILE.ukl")]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if args.file.is_none() {
        println!(
            "{} {}",
            ">".blue().bold(),
            "Entering REPL...".green().bold()
        );
        std::process::exit(1);
    }

    let compile_path = args.file.unwrap();

    println!(
        "{} {} {}",
        ">".blue().bold(),
        "Compiling file:".white(),
        compile_path.display().to_string().green().bold()
    );
}
