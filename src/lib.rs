use std::path::Path;

use clap::ValueEnum;
use config::constants::CacheStrategy;
use library::list;

mod compare;
mod config;
mod library;

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
  Txt,
  Json,
  Csv,
}

pub struct PublicConfig {
  pub path: String,
  pub similarity_threshold: f64,
  pub output_format: OutputFormat,
  pub output_all_scores: bool,
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
        similarity_threshold: args.similarity_threshold,
        output_format: args.output_format,
        output_all_scores: args.output_all_scores,
      },
      cache_strategy: CacheStrategy::Disk(Path::new("/tmp/img-compare")),
    })
  }
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  let images = list::list_images_recursively(Path::new(&config.user_config.path))?;
  let compare_results = compare::run::compare_all_images(&images, config)?;
  compare::output::save_output(&compare_results, config)?;

  Ok(())
}
