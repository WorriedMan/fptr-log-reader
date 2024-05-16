#[macro_use]
extern crate lazy_static;

use std::env;
use std::io::BufRead;
use std::process::ExitCode;

use chrono::TimeZone;
use crate::analyzer::print_stats;

use crate::reader::{open_file, parse_file};

mod models;
mod reader;
mod analyzer;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Укажите название файла");
        return ExitCode::FAILURE;
    }
    let reader = match open_file(&args[1]) {
        Err(e) => {
            println!("{e}");
            return ExitCode::FAILURE;
        }
        Ok(r) => r,
    };
    println!("Идет парсинг документа...");
    let parsed = match parse_file(reader.lines()) {
        Err(e) => {
            println!("{e}");
            return ExitCode::FAILURE;
        }
        Ok(p) => p
    };
    print_stats(parsed);
    ExitCode::SUCCESS
}
