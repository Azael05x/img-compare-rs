use std::process;

use clap::Parser;
use img_compare::PublicConfig;

#[derive(Parser)]
struct Args {
  #[clap(short, long)]
  folder: String,
  #[clap(short, long, default_value = "output.txt")]
  output: String,
  #[clap(short, long, default_value = "0.9")]
  similarity_threshold: String,
}

// TODO:
// Tests
// flag to include sub directories
//

fn main() {
  let args = Args::parse();

  let config = img_compare::Config::new(PublicConfig {
    path: args.folder.clone(),
    output: args.output.clone(),
    similarity_threshold: args.similarity_threshold.parse().unwrap_or(0.9),
  }).unwrap_or_else(|err| {
    eprintln!("Error: {}", err);
    process::exit(1);
  });

  if let Err(e) = img_compare::run(&config) {
    eprintln!("Error: {}", e);
    process::exit(1)
  };
}
