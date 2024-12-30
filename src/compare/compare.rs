use image::{self, imageops::FilterType, DynamicImage};
use image_compare::Algorithm;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::{
  fs,
  path::{Path, PathBuf},
};

use crate::{config::constants::CacheStrategy, Config};

/// Compare all images in the given list and return a list of similar images.
///
/// The comparison is done in parallel using Rayon, using multiple threads.
pub fn compare_all_images(
  images: &Vec<PathBuf>,
  config: &Config,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let pb = ProgressBar::new(images.len() as u64);

  let results = match config.cache_strategy {
    CacheStrategy::Disk(path) => compare_with_disk_cache(&images, &config, &pb, path),
    CacheStrategy::InMemory => compare_in_memory(&images, &config, &pb),
    CacheStrategy::LRU => panic!("Is not implemented yet"),
  }?;

  pb.finish_with_message("done");

  Ok(results)
}

/// Stores cached resized versions on disk for faster eventual lookups
fn compare_with_disk_cache(
  images: &Vec<PathBuf>,
  config: &Config,
  progress: &ProgressBar,
  path: &Path,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  fs::create_dir_all(path)
    .map_err(|e| format!("Error creating folder \"{}\": {}", path.display(), e))?;

  Ok(vec![])
}

/// Does all the job in memory, doesn't use cache or disk space
/// TODO: Need to open first iterator image outside `compare_two_images`
fn compare_in_memory(
  images: &Vec<PathBuf>,
  config: &Config,
  progress: &ProgressBar,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let result = images
    .par_iter()
    .enumerate()
    .flat_map(|(i, file_a)| {
      let result = images[(i + 1)..]
        .par_iter()
        .filter_map(|file_b| match compare_two_images(file_a, file_b) {
          Ok(similarity_score) if similarity_score > config.user_config.similarity_threshold => {
            Some(format!(
              "{:?} and {:?} are similar. Score: {:.2}\n",
              file_a, file_b, similarity_score
            ))
          }
          Ok(_) => None,
          Err(err) => {
            eprintln!("{}", err);
            None
          }
        })
        .collect::<Vec<_>>();

      progress.inc(1);

      result
    })
    .collect::<Vec<_>>();

  Ok(result)
}

fn compare_two_images(image_one: &PathBuf, image_two: &PathBuf) -> Result<f64, String> {
  let image_one_file = image::open(image_one)
    .map_err(|err| format!("Error loading image {}: {}", image_one.display(), err))?;
  let image_two_file = image::open(image_two)
    .map_err(|err| format!("Error loading image {}: {}", image_two.display(), err))?;

  if !super::aspect_ratio::compare_aspect_ratios(&image_one_file, &image_two_file) {
    return Ok(0.0);
  }

  let resized_image_one = image_one_file
    .resize_exact(100, 100, FilterType::Nearest)
    .into_luma8();
  let resized_image_two = image_two_file
    .resize_exact(100, 100, FilterType::Nearest)
    .into_luma8();

  image_compare::gray_similarity_structure(
    &Algorithm::MSSIMSimple,
    &resized_image_one,
    &resized_image_two,
  )
  .map(|result| result.score)
  .map_err(|err| {
    format!(
      "Error comparing images {} and {}: {}",
      image_one.display(),
      image_two.display(),
      err
    )
  })
}
