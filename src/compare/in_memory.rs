use image;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::{error::Error, path::PathBuf};

use crate::Config;

/// Does all the job in memory, doesn't use cache or disk space
pub fn compare_in_memory(
  images: &[PathBuf],
  config: &Config,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let pb = ProgressBar::new(images.len() as u64);

  let result: Result<Vec<_>, _> = images
    .par_iter()
    .enumerate()
    .map(|(i, file_a)| {
      // Load image buffer once and provide it to comparison method
      let image_one_file = image::open(file_a)
        .map_err(|err| format!("Error loading image {}: {}", file_a.display(), err))?;
      let resized_image_one = super::compare::image_to_processed_gray_image(image_one_file);

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
          let resized_image_two = super::compare::image_to_processed_gray_image(image_two_file);

          match super::compare::compare_two_processed_gray_images(
            &resized_image_one,
            &resized_image_two,
          ) {
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

      pb.inc(1);

      comparisons
    })
    .collect::<Result<Vec<_>, _>>()
    .map_err(Box::<dyn Error>::from);

  pb.finish_with_message("done");

  // Flatten the nested results and propagate errors
  result.map(|vecs| vecs.into_iter().flatten().collect())
}
