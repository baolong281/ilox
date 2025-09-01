use std::{
    io::{self, BufRead, Write},
    process::exit,
};

use crate::scanner::{self};

pub fn run_file(file: String) -> () {
    let contents = std::fs::read_to_string(&file).unwrap_or_else(|err| {
        eprintln!("Could not read file {}: {}", file, err);
        exit(74);
    });
}

pub fn run_repl() -> () {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let stdout = io::stdout();
    let mut out = stdout.lock();

    loop {
        out.write("> ".as_bytes()).unwrap();
        out.flush().unwrap();

        let mut line = String::new();
        let bytes = handle.read_line(&mut line).unwrap();

        if bytes == 0 {
            break;
        }

        run(line);
    }
}

pub fn run(code: String) -> () {
    let mut scanner = scanner::Scanner::new(code);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        match token {
            scanner::ScannerResult::Token(token) => {
                println!("{:?}", token);
            }
            scanner::ScannerResult::Error(error) => {
                println!("{:?}", error);
            }
        }
    }
}
