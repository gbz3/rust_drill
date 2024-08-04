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
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(fr) => {
                let mut empty_num = 0;
                for (line_num, line) in fr.lines().enumerate() {
                    let line = line?;
                    match config {
                        Config {number_lines: true, ..} => println!("{0: >6}\t{1}", line_num + 1, line),
                        Config {number_nonblank_lines: true, ..} => {
                            match line.is_empty() {
                                true => {
                                    empty_num += 1;
                                    println!();
                                }
                                false => println!("{0: >6}\t{1}", line_num + 1 - empty_num, line)
                            }
                        },
                        _ => println!("{}", line),
                    }
                }
            },
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
