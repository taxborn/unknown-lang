use std::path::PathBuf;
use clap::Parser;

pub mod lexer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to compile. If no file is provided, enter unknown-lang REPL.
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>
}

fn main() {
    let args = Args::parse();

    if let Some(compile_path) = args.file.as_deref() {
        println!("Compiling file: {}", compile_path.display());
    }

    println!("Hello, world!");
}
