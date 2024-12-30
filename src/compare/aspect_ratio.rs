use image::GenericImageView;
use image::{self, DynamicImage};

const ASPECT_RATIO_TOLERANCE: f64 = 0.01;

pub fn compare_aspect_ratios(image_a: &DynamicImage, image_b: &DynamicImage) -> bool {
  let aspect_ratio_a = calculate_aspect_ratio(image_a);
  let aspect_ratio_b = calculate_aspect_ratio(image_b);

  // Use a tolerance for comparison
  (aspect_ratio_a - aspect_ratio_b).abs() < ASPECT_RATIO_TOLERANCE
}

fn calculate_aspect_ratio(image: &DynamicImage) -> f64 {
  let (width, height) = image.dimensions();

  // Calculate the aspect ratio and round it to two decimal points
  ((width as f64 / height as f64) * 100.0).round() / 100.0
}
