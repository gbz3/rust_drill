use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(value_name = "FILE", default_value = "-",
        help = "Input file(s) [default: -]")]
    files: Vec<String>,

    #[arg(group = "nc", short = 'n', long = "lines", default_value = "10",
        help = "print the first K lines instead of the first 10;")]
    lines: usize,

    #[arg(group = "nc", short = 'c', long = "bytes",
        help = "print the first K bytes of each file;")]
    bytes: Option<usize>,

}

pub fn get_args() -> MyResult<Config> {
    //Ok(Config::parse())
    let config = Config::parse();
    match config {
        Config { lines: 0, ..} => Err(From::from("illegal line count -- 0")),
        Config { bytes: Some(0), ..} => Err(From::from("illegal byte count -- 0")),
        _ => Ok(config),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("head: {}: {}", filename, err),
            Ok(file) => {
                if config.files.len() >= 2 { println!("==> {} <==", filename) }
                match config {
                    Config { bytes: Some(limit), ..} => {
                        for (byte_num, byte) in file.bytes().enumerate() {
                            if byte_num >= limit { break }
                            let byte = byte?;
                            let my_char = byte as char;
                            print!("{}", my_char);
                        }
                        if config.files.len() >= 2 { println!() }
                    },
                    _ => {
                        let mut printed_num = 0;
                        for (line_num, line) in file.lines().enumerate() {
                            if line_num >= config.lines { break }
                            let line = line?;
                            println!("{}", line);
                            printed_num += 1;
                        }
                        if printed_num > config.lines { println!() }
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
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[allow(dead_code)]
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
