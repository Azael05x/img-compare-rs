use std::{fs::File, io::Write, path::Path};

mod list;
use list::images;

mod compare;
mod config;

pub struct PublicConfig {
  pub path: String,
  pub output: String,
  pub similarity_threshold: f64,
}

pub struct Config {
  user_config: PublicConfig,
  // progress_bar_style: indicatif::style::ProgressStyle,
}

impl Config {
  pub fn new(args: PublicConfig) -> Result<Config, &'static str> {
    let similarity_threshold = args.similarity_threshold;
    if similarity_threshold < 0.0 || similarity_threshold > 1.0 {
      return Err("Similarity threshold must be between 0.0 and 1.0");
    }

    Ok(Config {
      user_config: PublicConfig {
        path: args.path,
        output: args.output,
        similarity_threshold: args.similarity_threshold,
      },
      // progress_bar_style: ProgressStyle::default_bar(),
    })
  }
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  let images = images::list_images_recursively(Path::new(&config.user_config.path))?;
  let compare_results = compare::compare::compare_all_images(&images, config);

  // Write all results to the output file
  let mut output = File::create(&config.user_config.output)?;
  for result in compare_results {
    writeln!(output, "{}", result)?;
  }

  Ok(())
}
