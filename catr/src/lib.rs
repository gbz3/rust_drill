use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(value_name = "FILE", help = "Input file(s) [default: -]", default_value = "-")]
    files: Vec<String>,

    #[arg(group = "nb", short = 'n', long = "number", help = "number all output lines")]
    number_lines: bool,

    #[arg(group = "nb", short = 'b', long = "number-nonblank", help = "number nonempty output lines, overrides -n")]
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(fr) => {
                let mut n = 0;
                let mut b = 0;
                for line in fr.lines().map(|l| l.unwrap()) {
                    match config {
                        Config {number_lines: true, ..} => {
                            n = n + 1;
                            println!("{0: >6}\t{1}", n, line)
                        },
                        Config {number_nonblank_lines: true, ..} => {
                            if !line.is_empty() {
                                b = b + 1;
                                println!("{0: >6}\t{1}", b, line)
                            }
                            else { println!() }
                        },
                        _ => println!("{}", line),
                    }
                }
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
