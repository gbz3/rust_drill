use std::error::Error;
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(value_name = "FILE", help = "Input file(s) [default: -]", default_value = "-")]
    files: Vec<String>,

    #[arg(group = "nc", short = 'n', long = "lines", help = "print the first K lines instead of the first 10;")]
    lines: usize,

    #[arg(group = "nc", short = 'c', long = "bytes", help = "print the first K bytes of each file;")]
    bytes: Option<usize>,

}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);

    Ok(())
}
