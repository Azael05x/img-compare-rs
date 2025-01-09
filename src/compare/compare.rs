use image::{imageops::FilterType, DynamicImage, GrayImage};
use image_compare::Algorithm;

pub fn compare_two_processed_gray_images(
  image_one_file: &GrayImage,
  image_two_file: &GrayImage,
) -> Result<f64, String> {
  image_compare::gray_similarity_structure(&Algorithm::MSSIMSimple, image_one_file, image_two_file)
    .map(|result| result.score)
    .map_err(|err| format!("Error comparing images: {}", err))
}

/// Transform image to specific size and style for comparison
pub fn image_to_processed_gray_image(image: DynamicImage) -> GrayImage {
  image
    .resize_exact(100, 100, FilterType::Nearest)
    .into_luma8()
}
