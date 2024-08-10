use std::error::Error;
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

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);

    Ok(())
}
