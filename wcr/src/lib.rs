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

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
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
    let mut num_files = 0;
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            //Ok(_) => println!("Opened {}", filename),
            Ok(br) => {
                num_files += 1;
                match count(br) {
                    Err(err) => eprintln!("{}: {}", filename, err),
                    Ok(fi) => {
                        total_lines += fi.num_lines;
                        total_words += fi.num_words;
                        total_bytes += fi.num_bytes;
                        total_chars += fi.num_chars;
                        if config.lines { print!("{:>8}", fi.num_lines) }
                        if config.words { print!("{:>8}", fi.num_words) }
                        if config.bytes { print!("{:>8}", fi.num_bytes) }
                        if config.chars { print!("{:>8}", fi.num_chars) }
                        if filename == "-" { println!() }
                        else { println!(" {}", filename) }
                    }
                }
            }
        }
    }
    if num_files >= 2 {
        if config.lines { print!("{:>8}", total_lines) }
        if config.words { print!("{:>8}", total_words) }
        if config.bytes { print!("{:>8}", total_bytes) }
        if config.chars { print!("{:>8}", total_chars) }
        println!(" total")
    }

    Ok(())
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_chars = 0;
    let mut num_bytes = 0;
    let mut num_words = 0;
    let mut num_lines = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 { break }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

}