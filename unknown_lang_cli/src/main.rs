use clap::Parser;
use colored::*;
use std::path::PathBuf;

use unknown_lang_parser;

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

        unimplemented!("REPL is not implemented yet. Run with a specific file \
            passed with --file.");
    }

    // We can unwrap since we check for the None case before we get here
    let compile_path = args.file.unwrap();

    println!(
        "{} {} {}",
        ">".blue().bold(),
        "Compiling file:".white(),
        compile_path.display().to_string().green().bold()
    );
}
