#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;

mod solvers;

use clap::{App, Arg};
use solvers::backtracking::Sudoku;
use std::fs;

fn main() {
    env_logger::init();
    let mut s = match load_sudoku_from_file(&get_sudoku_path()) {
        Ok(path) => path,
        Err(err) => {
            error!("Cannot load sudoku from file: {}", err.msg);
            return;
        }
    };
    println!("Solving sudoku");
    println!("{}", s);
    match s.solve() {
        Ok(_) => {
            println!("Solved!");
            println!("{}", s);
        }
        Err(_) => println!("Cannot solve sudoku"),
    }
}

#[derive(Debug)]
struct LoadingError {
    msg: String,
}

fn load_sudoku_from_file(file_path: &str) -> Result<Sudoku, LoadingError> {
    let data = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(err) => {
            return Err({
                LoadingError {
                    msg: err.to_string(),
                }
            })
        }
    };
    let clean_data: Vec<u8> = data
        .chars()
        .filter(|c| c.to_digit(10).is_some())
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    match Sudoku::new(clean_data.into_iter()) {
        Some(sudoku) => Ok(sudoku),
        None => Err(LoadingError {
            msg: "Cannot create sudoku from data".to_string(),
        }),
    }
}

fn get_sudoku_path() -> String {
    let matches = App::new("Sudoku solver")
        .version("0.1.0")
        .author("Yuriy Senko <yura.senko@gmail.com>")
        .arg(
            Arg::with_name("sudoku_path")
                .short("s")
                .long("--sudoku-path")
                .takes_value(true)
                .required(true)
                .help("File with the task"),
        )
        .get_matches();
    matches.value_of("sudoku_path").unwrap().to_string()
}
