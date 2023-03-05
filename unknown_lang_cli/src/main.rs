use clap::Parser;
use colored::*;
use std::{fs, path::PathBuf};

use unknown_lang_parser::lexer::{state::Lexer, tokens::Token};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to compile. If no file is provided, enter unknown-lang REPL.
    #[arg(short, long, value_name = "FILE.ukl")]
    file: Option<PathBuf>,
    /// Toggle to print the tokens of the file. Needs --file to be passed.
    #[arg(short, long, default_value_t = false)]
    print_tokens: bool,
    /// Toggle to print diagnostics of compilation, like timings.
    #[arg(short, long, default_value_t = false)]
    diagnostics: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.file.is_none() {
        println!(
            "{} {}",
            ">".blue().bold(),
            "Entering REPL...".green().bold()
        );

        unimplemented!(
            "REPL is not implemented yet. Run with a specific file \
            passed with --file."
        );
    }

    // We can unwrap since we check for the None case before we get here
    let compile_path = args.file.unwrap();
    let file_contents = fs::read_to_string(&compile_path)?;

    println!(
        "{} {} {}",
        ">".blue().bold(),
        "Compiling file:".white(),
        compile_path.display().to_string().green().bold()
    );

    let mut lexer = Lexer::new(&file_contents);

    //-----------------------------
    // This section currently is for debug purposes, this will be removed and
    // substituted in for a parser.

    let mut tok = lexer.lex_next()?;
    let mut count = 0;
    let start = std::time::Instant::now();

    // loop through the tokens
    while tok != Token::Eof {
        count += 1;
        if args.print_tokens {
            println!("{tok:?}");
        }

        tok = lexer.lex_next()?;
    }

    let end = start.elapsed();

    if args.diagnostics {
        println!(
            "{} {} {} {} {:?}{}",
            ">".blue().bold(),
            "Lexing".green(),
            count,
            "tokens took".green(),
            end,
            ".".green()
        );
    }

    //-----------------------------

    println!(
        "{} {}",
        ">".blue().bold(),
        "Compilation successful!".green(),
    );

    Ok(())
}
