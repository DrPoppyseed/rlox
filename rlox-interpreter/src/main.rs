#![allow(dead_code)]
extern crate alloc;

mod scanner;
mod token;

use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
    path::PathBuf,
    process,
};

use scanner::Scanner;

#[derive(Debug, Clone)]
enum CliCommands {
    Repl,
    File,
}

#[derive(Debug, Clone)]
enum ReplKeywords {
    Quit,
    Exit,
    Clean,
}

impl TryFrom<String> for ReplKeywords {
    type Error = CliError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "quit" => Ok(ReplKeywords::Quit),
            "exit" => Ok(ReplKeywords::Exit),
            "clean" => Ok(ReplKeywords::Clean),
            value => Err(CliError::NotReplKeyword(value.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
enum CliError {
    InvalidCommand(String),
    NotReplKeyword(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::InvalidCommand(msg) => {
                write!(f, "invalid command. {}", msg)
            }
            CliError::NotReplKeyword(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl TryFrom<String> for CliCommands {
    type Error = CliError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "repl" => Ok(CliCommands::Repl),
            "file" => Ok(CliCommands::File),
            value => {
                Err(Self::Error::InvalidCommand(format!("received '{value}'")))
            }
        }
    }
}

pub fn main() {
    let mut command = CliCommands::Repl;

    if let Some(unsafe_command) = std::env::args().nth(1) {
        match CliCommands::try_from(unsafe_command) {
            Ok(com) => command = com,
            Err(err) => {
                eprintln!("{}", err);
                eprintln!("program exited with code 64.");
                process::exit(64);
            }
        }
    }

    match command {
        CliCommands::File => match std::env::args().nth(2) {
            Some(path) => run_file(path.into()),
            None => {
                eprintln!("expected filepath. none was found.");
                eprintln!("program exited with code 64.");
                process::exit(64);
            }
        },
        CliCommands::Repl => run_repl(),
    }
}

fn run_repl() {
    println!("welcome to rlox");

    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        let repl = stdin();
        repl.read_line(&mut input).unwrap();

        println!("{}", input);
    }
}

fn run_file(path: PathBuf) {
    println!("file! {:?}", path);
}

fn run(source: &str) {
    let mut scan = Scanner::new(source);
    let tokens = scan.try_scan_tokens().unwrap();

    let _ = tokens.iter().map(|token| {
        println!("{:?}", token);
    });
}
