use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(value_name = "FILE", default_value = "-",
        help = "Input file(s)")]
    files: Vec<String>,

    #[arg(short = 'l', long = "lines",
        help = "Show line count")]
    lines: bool,

    #[arg(short = 'w', long = "words",
        help = "Show word count")]
    words: bool,

    #[arg(group = "cm", short = 'c', long = "bytes",
        help = "Show byte count")]
    bytes: bool,

    #[arg(group = "cm", short = 'm', long = "chars",
        help = "Show character count")]
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    // Ok(Config::parse())
    let mut config = Config::parse();
    match config {
        Config {lines: false, words: false, bytes: false, chars: false, ..} => {
            config.lines = true;
            config.words = true;
            config.bytes = true;
        },
        _ => ()
    }
    Ok(config)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    //println!("{:#?}", config);
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }

    Ok(())
}
