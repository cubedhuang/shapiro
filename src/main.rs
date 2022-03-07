use std::io::Write;

use parser::Parser;
use token::Location;

use crate::walker::Walker;

mod ast;
mod lexer;
mod parser;
mod token;
mod walker;

fn main() {
    ctrlc::set_handler(bye).unwrap();

    let args = std::env::args();

    match args.len() - 1 {
        0 => repl(),
        _ => todo!(),
    }
}

fn repl() {
    let mut buffer = String::new();
    let input = std::io::stdin();
    let walker = Walker::new();

    loop {
        print!("\nshap> ");
        std::io::stdout().flush().unwrap();

        buffer.clear();
        input.read_line(&mut buffer).unwrap();

        match buffer.trim() {
            "exit" => bye(),
            _ => run(&buffer[0..buffer.len() - 2], &walker),
        }
    }
}

fn run(input: &str, walker: &Walker) {
    let parser = Parser::new(input);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            report_error(input, e.loc(), &format!("{e}"));
            return;
        }
    };

    for stmt in ast {
        if let Err(e) = walker.eval(stmt) {
            report_error(input, e.loc(), &format!("{e}"));
            return;
        }
    }
}

fn report_error(input: &str, loc: Location, message: &str) {
    dbg!(input);
    for l in input.lines() {
        dbg!(l);
    }

    let line = match input.lines().nth(loc.line - 1) {
        Some(line) => line,
        None => "",
    };

    eprintln!("[{line}:{col}] {message}", line = loc.line, col = loc.col);

    eprintln!("{line}");

    for _ in 0..loc.col - 1 {
        eprint!(" ");
    }

    eprint!("^ here");
}

fn bye() {
    println!("\nFacts don't care about your feelings.");
    std::process::exit(0);
}
