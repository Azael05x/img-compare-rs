use std::{fs::File, io::Write, path::Path};

use config::constants::CacheStrategy;
use library::list;

mod compare;
mod config;
mod library;

pub struct PublicConfig {
  pub path: String,
  pub output: String,
  pub similarity_threshold: f64,
}

pub struct Config {
  user_config: PublicConfig,
  cache_strategy: CacheStrategy, // progress_bar_style: indicatif::style::ProgressStyle,
}

impl Config {
  pub fn new(args: PublicConfig) -> Result<Config, &'static str> {
    let similarity_threshold = args.similarity_threshold;
    if !(0.0..=1.0).contains(&similarity_threshold) {
      return Err("Similarity threshold must be between 0.0 and 1.0");
    }

    Ok(Config {
      user_config: PublicConfig {
        path: args.path,
        output: args.output,
        similarity_threshold: args.similarity_threshold,
      },
      cache_strategy: CacheStrategy::Disk(Path::new(".cache")),
      // cache_strategy: CacheStrategy::InMemory,
      // progress_bar_style: ProgressStyle::default_bar(),
    })
  }
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  let images = list::list_images_recursively(Path::new(&config.user_config.path))?;
  let compare_results = compare::run::compare_all_images(&images, config)?;

  // Write all results to the output file
  let mut output = File::create(&config.user_config.output)?;
  for result in compare_results {
    writeln!(output, "{}", result)?;
  }

  Ok(())
}
