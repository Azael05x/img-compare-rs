use image::{self, imageops::FilterType, DynamicImage, GrayImage};
use image_compare::Algorithm;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::{
  error::Error,
  fs,
  path::{Path, PathBuf},
};

use crate::{config::constants::CacheStrategy, Config};

/// Compare all images in the given list and return a list of similar images.
///
/// The comparison is done in parallel using Rayon, using multiple threads.
pub fn compare_all_images(
  images: &[PathBuf],
  config: &Config,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let pb = ProgressBar::new(images.len() as u64);

  let results = match config.cache_strategy {
    CacheStrategy::Disk(path) => compare_with_disk_cache(images, config, &pb, path),
    CacheStrategy::InMemory => compare_in_memory(images, config, &pb),
    CacheStrategy::Lru => panic!("Is not implemented yet"),
  }?;

  pb.finish_with_message("done");

  Ok(results)
}

/// Stores cached resized versions on disk for faster eventual lookups
fn compare_with_disk_cache(
  images: &[PathBuf],
  config: &Config,
  progress: &ProgressBar,
  path: &Path,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  fs::create_dir_all(path)
    .map_err(|e| format!("Error creating folder \"{}\": {}", path.display(), e))?;

  // Load image from disk or prepare and save on disk
  // What is best format to save image to?
  // match resized_image_one.save("/Users/aigarscibulskis-work/projects/img-compare/.cache/test.png") {
  //   Ok(_) => (),
  //   Err(err) => eprintln!("{}", err),
  // }

  // Pass buffer to image comparison method

  Ok(vec![])
}

/// Does all the job in memory, doesn't use cache or disk space
fn compare_in_memory(
  images: &[PathBuf],
  config: &Config,
  progress: &ProgressBar,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let result: Result<Vec<_>, _> = images
    .par_iter()
    .enumerate()
    .map(|(i, file_a)| {
      // Load image buffer once and provide it to comparison method
      let image_one_file = image::open(file_a)
        .map_err(|err| format!("Error loading image {}: {}", file_a.display(), err))?;
      let resized_image_one = transform_image_to_processed_gray_image(image_one_file);

      let comparisons: Result<Vec<_>, _> = images[(i + 1)..]
        .par_iter()
        .filter_map(|file_b| {
          let image_two_file = match image::open(file_b) {
            Ok(img) => img,
            Err(err) => {
              return Some(Err(format!(
                "Error loading image {}: {}",
                file_b.display(),
                err
              )))
            }
          };
          let resized_image_two = transform_image_to_processed_gray_image(image_two_file);

          match compare_two_processed_images(&resized_image_one, &resized_image_two) {
            Ok(similarity_score) if similarity_score > config.user_config.similarity_threshold => {
              Some(Ok(format!(
                "{:?} and {:?} are similar. Score: {:.2}\n",
                file_a, file_b, similarity_score
              )))
            }
            Ok(_) => None,
            Err(err) => Some(Err(err)),
          }
        })
        .collect();

      progress.inc(1);

      comparisons
    })
    .collect::<Result<Vec<_>, _>>()
    .map_err(Box::<dyn Error>::from);

  progress.finish_with_message("done");

  // Flatten the nested results and propagate errors
  result.map(|vecs| vecs.into_iter().flatten().collect())
}

/// Transform image to specific size and style for comparison
fn transform_image_to_processed_gray_image(image: DynamicImage) -> GrayImage {
  image
    .resize_exact(100, 100, FilterType::Nearest)
    .into_luma8()
}

fn compare_two_processed_images(
  image_one_file: &GrayImage,
  image_two_file: &GrayImage,
) -> Result<f64, String> {
  image_compare::gray_similarity_structure(
    &Algorithm::MSSIMSimple,
    image_one_file,
    image_two_file,
  )
  .map(|result| result.score)
  .map_err(|err| format!("Error comparing images: {}", err))
}
