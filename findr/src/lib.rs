use std::error::Error;
use clap::{Parser, ValueEnum};
use clap::builder::PossibleValue;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(value_name = "PATH", default_value = ".",
        help = "Search paths")]
    paths: Vec<String>,

    #[arg(short = 'n', long = "name", value_name = "NAME", num_args = 1..,
        help = "Name")]
    names: Vec<Regex>,

    #[arg(short = 't', long = "type", value_name = "TYPE", num_args = 1..,
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

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
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
    let is_match = |entry: &DirEntry| -> bool {
        config.entry_types.is_empty() ||
            (config.entry_types.contains(&EntryType::Dir) && entry.file_type().is_dir()) ||
            (config.entry_types.contains(&EntryType::File) && entry.file_type().is_file()) ||
            (config.entry_types.contains(&EntryType::Link) && entry.file_type().is_symlink())
    };

    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                //Ok(entry) => println!("{}", entry.path().display()),
                Ok(entry) => {
                    if is_match(&entry) { println!("{}", entry.path().display()) }
                },
            }
        }
    }

    Ok(())
}
