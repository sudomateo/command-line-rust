use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("sudomateo")
        .about("A cat clone.")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(false)
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("number_lines")
                .long("--number")
                .short("n")
                .help("Number all lines in the output")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .long("--number-nonblank")
                .short("b")
                .help("Number only nonblank lines in the output")
                .takes_value(false)
                .conflicts_with("number_lines"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        let fh = match open(&filename) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open {}: {}", filename, e);
                return Ok(());
            }
        };

        let mut line_number = 1;

        for line in fh.lines() {
            let line = line?;

            if config.number_lines {
                println!("{:>6}\t{}", line_number, line);
            } else if config.number_nonblank_lines {
                if line.is_empty() {
                    println!();
                    continue;
                }
                println!("{:>6}\t{}", line_number, line);
            } else {
                println!("{}", line);
            }

            line_number += 1
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
