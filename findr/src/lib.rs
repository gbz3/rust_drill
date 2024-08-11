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
    let match_types = |entry: &DirEntry| -> bool {
        config.entry_types.is_empty() ||
            config.entry_types.iter().any(|entry_type| {
                match entry_type {
                    EntryType::Dir => entry.file_type().is_dir(),
                    EntryType::File => entry.file_type().is_file(),
                    EntryType::Link => entry.file_type().is_symlink(),
                }
            })
    };

    let match_names = |entry: &DirEntry| -> bool {
        config.names.is_empty() ||
            config.names.iter().any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in config.paths {
        WalkDir::new(path).into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                },
                Ok(entry) => Some(entry),
            })
            .filter(match_types)
            .filter(match_names)
            .map(|entry| entry.path().display().to_string())
            .for_each(|e| println!("{}", e));
    }

    Ok(())
}
