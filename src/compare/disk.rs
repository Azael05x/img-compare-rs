use image::{self, DynamicImage, GrayImage};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::{
  error::Error,
  fs,
  path::{Path, PathBuf},
};

use crate::Config;

use super::output::Output;

/// Stores cached resized versions on disk for faster eventual lookups
pub fn compare_with_disk_cache(
  images: &[PathBuf],
  config: &Config,
  cache_path: &Path,
) -> Result<Vec<Output>, Box<dyn Error>> {
  // Create folder for cached images
  fs::create_dir_all(cache_path)
    .map_err(|e| format!("Error creating folder \"{}\": {}", cache_path.display(), e))?;

  let pb = ProgressBar::new(images.len() as u64);

  let results: Vec<Output> = images
    .par_iter()
    .enumerate()
    .flat_map(|(i, image_one_path)| {
      let image_one_file = match get_cached_resized_image(image_one_path, cache_path) {
        Ok(image) => image,
        Err(err) => {
          eprintln!(
            "Couldn't open {}: {}. Skipping.. ",
            image_one_path.display(),
            err
          );
          return std::iter::empty().collect::<Vec<_>>().into_par_iter();
        }
      };

      let file_results: Vec<Output> = images[(i + 1)..].par_iter().filter_map(|image_two_path| {
        let image_two_file = match get_cached_resized_image(image_two_path, cache_path) {
          Ok(image) => image,
          Err(err) => {
            eprintln!(
              "Couldn't open {}: {}. Skipping.. ",
              image_two_path.display(),
              err
            );
            return None;
          }
        };

        let score =
          super::compare::compare_two_processed_gray_images(&image_one_file, &image_two_file)
            .unwrap();

        Some(Output {
          file_name_1: image_one_path.display().to_string(),
          file_name_2: image_two_path.display().to_string(),
          similarity_score: score,
        })
      }).collect();

      pb.inc(1);

      file_results.into_par_iter()
    }).collect();

  pb.finish();

  Ok(results)
}

fn get_cached_resized_image(
  image_path: &PathBuf,
  cache_path: &Path,
) -> Result<GrayImage, Box<dyn Error>> {
  let hash = format!(
    "{:x}",
    md5::compute(image_path.to_string_lossy().as_bytes())
  );
  let cached_path = cache_path.join(format!("{}.png", hash));

  if cached_path.exists() {
    // Load from cache
    let cached_image = image::open(&cached_path)?;

    return match cached_image {
      DynamicImage::ImageLuma8(image) => Ok(image),
      _ => panic!("Cached image is not grayscale!"),
    };
  }

  let image = image::open(&image_path)?;
  let resized = super::compare::image_to_processed_gray_image(image);
  resized.save(&cached_path)?;
  Ok(resized)
}
