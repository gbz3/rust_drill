use std::error::Error;
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(value_name = "FILE", help = "Input file(s) [default: -]", default_value = "-")]
    files: Vec<String>,

    #[arg(short = 'n', long = "number", help = "number all output lines")]
    number_lines: bool,

    #[arg(short = 'b', long = "number-nonblank", help = "number nonempty output lines, overrides -n")]
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);

    Ok(())
}
