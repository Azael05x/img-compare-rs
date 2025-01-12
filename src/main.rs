use std::process;

use clap::Parser;
use img_compare::{OutputFormat, PublicConfig};

#[derive(Parser)]
struct Args {
  #[clap(short, long)]
  folder: String,
  #[clap(short, long, default_value = "0.9")]
  similarity_threshold: String,
  #[clap(long, value_enum, default_value = "json")]
  output_format: OutputFormat,
  #[clap(long, action)]
  output_all_scores: bool,
}

// TODO:
// Tests
// flag to include sub directories
//

fn main() {
  let args = Args::parse();

  let config = img_compare::Config::new(PublicConfig {
    similarity_threshold: args.similarity_threshold.parse().unwrap_or(0.9),

    path: args.folder,

    output_format: args.output_format,
    output_all_scores: args.output_all_scores,
  })
  .unwrap_or_else(|err| {
    eprintln!("Error: {}", err);
    process::exit(1);
  });

  if let Err(e) = img_compare::run(&config) {
    eprintln!("Error: {}", e);
    process::exit(1)
  };
}
