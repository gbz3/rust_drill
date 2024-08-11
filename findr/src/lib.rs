use std::error::Error;
use clap::{Parser, ValueEnum};
use clap::builder::PossibleValue;
use regex::Regex;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(value_name = "PATH", default_value = ".",
        help = "Search paths")]
    paths: Vec<String>,

    #[arg(short = 'n', long = "name", value_name = "NAME",
        help = "Name")]
    names: Vec<Regex>,

    #[arg(short = 't', long = "type", value_name = "TYPE",
    help = "Entry type")]
    #[clap(value_enum)]
    entry_types: Vec<EntryType>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    //println!("{:#?}", config);
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }

    Ok(())
}
