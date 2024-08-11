use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(value_name = "IN_FILE", default_value = "-",
        help = "Input file")]
    in_file: String,

    #[arg(value_name = "OUT_FILE",
        help = "Output file")]
    out_file: Option<String>,

    #[arg(short = 'c', long = "count",
        help = "Show counts")]
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    //println!("{:#?}", config);
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    let mut last = String::new();
    let mut last_count = 0;

    let bytes = file.read_line(&mut line)?;
    if bytes == 0 { return Ok(()) }
    last.clone_from(&line);

    loop {
        if last != line {
            if config.count { print!("{:>4} {}", last_count, last) }
            else { print!("{}", last) }
            last.clone_from(&line);
            last_count = 1;
        }
        else { last_count += 1 }

        line.clear();
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 { break }
    }
    if config.count { print!("{:>4} {}", last_count, last) }
    else { print!("{}", last) }

    Ok(())
}
