use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let mut last_idx = 0;
                for (idx, line) in file.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:6}\t{}", idx + 1, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!()
                        } else {
                            last_idx += 1;
                            println!("{:6}\t{}", last_idx, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("candy12t")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .default_value("-")
                .multiple(true)
                .help("Input file(s)"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number-nonblank"),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number nonblank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
