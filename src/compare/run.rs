use std::path::PathBuf;

use crate::{config::constants::CacheStrategy, Config};

/// Compare all images in the given list and return a list of similar images.
///
/// The comparison is done in parallel using Rayon, using multiple threads.
pub fn compare_all_images(
  images: &[PathBuf],
  config: &Config,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let results = match config.cache_strategy {
    CacheStrategy::Disk(path) => super::disk::compare_with_disk_cache(images, config, path),
    CacheStrategy::InMemory => super::in_memory::compare_in_memory(images, config),
  }?;

  Ok(results)
}
