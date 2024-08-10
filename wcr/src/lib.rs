use std::error::Error;
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

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);

    Ok(())
}
