mod expr;
mod lox;
mod parser;
mod scanner;

use std::process::exit;

use crate::lox::{run_file, run_repl};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: <program> [repl|file]");
        exit(64);
    }

    if args[1] == "repl" {
        run_repl();
    } else {
        run_file(args[1].clone());
    }
}
