mod loundness;

use std::path::PathBuf;

use clap::Parser;
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use walkdir::{DirEntry, WalkDir};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// The threshold LUFS value.
    /// The given value is transformed to a negative value.
    /// For example, a value of 14.0 is transformed to -14.0 LUFS.
    #[arg(value_parser = threshold_validator)]
    threshold: f64,

    /// File or directory path.
    base: PathBuf,

    /// The glob pattern to match files against.
    #[arg(short = 'i', long = "include", value_name = "PATTERN")]
    includes: Vec<String>,

    /// The glob pattern to exclude files against.
    #[arg(short = 'e', long = "exclude", value_name = "PATTERN")]
    excludes: Vec<String>,

    /// Do not ignore case distinctions in patterns.
    #[arg(long = "no-ignore-case")]
    case_sensitive: bool,

    /// Toggle verbosity
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

fn threshold_validator(
    val: &str,
) -> Result<f64, String> {
    match val.parse::<f64>() {
        Ok(val) => Ok(-val.abs()),
        _ => Err("threshold must be a decimal".into()),
    }
}

fn create_globset(
    globs: &[String],
    case_sensitive: bool,
) -> Result<GlobSet, Box<dyn std::error::Error>> {
    let mut globset_builder = GlobSetBuilder::new();

    for glob_pat in globs {
        let glob = GlobBuilder::new(glob_pat)
            .case_insensitive(!case_sensitive)
            .build()?;
        globset_builder.add(glob);
    }
    Ok(globset_builder.build()?)
}

fn create_entry_predicate(
    config: &Config,
) -> Result<impl Fn(&DirEntry) -> bool, Box<dyn std::error::Error>> {
    let include = create_globset(&config.includes, config.case_sensitive)?;
    let exclude = create_globset(&config.excludes, config.case_sensitive)?;

    return Ok(move |entry: &DirEntry| {
        let path = entry.path();

        if path.is_dir() {
            !exclude.is_match(path)
        } else {
            !exclude.is_match(path) && include.is_match(path)
        }
    })
}

fn process_entry(
    config: &Config,
    entry: &DirEntry,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = entry.path();

    if path.is_file() {
        let global_loundness = loundness::global(path)?;

        if config.verbose {
            eprintln!("{} global loudness is {}",
                path.display(),
                global_loundness,
            );
        }

        if global_loundness < config.threshold {
            println!("{}", path.display());
        }
    }
    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let walker = WalkDir::new(config.base.as_path()).into_iter();
    let keep_entry = create_entry_predicate(&config)?;

    for entry in walker.filter_entry(keep_entry) {
        if let Ok(entry) = entry {
            process_entry(&config, &entry)?;
        }
    }
    Ok(())
}

fn main() {
    let config = Config::parse();

    if !config.base.exists() {
        eprintln!("Error: {} does not exist", config.base.display());
        std::process::exit(1);
    }

    if let Err(e) = run(config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
