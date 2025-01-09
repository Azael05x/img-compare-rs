use std::path::Path;

use crate::config::constants::IMAGE_EXTENSIONS;

/**
* Check if the given path is a file with an image extension.
* Supported extensions based on the IMAGE_EXTENSIONS constant.
*/
pub fn is_image_file(path: &Path) -> bool {
  path.is_file()
    && path
      .extension()
      .and_then(|ext| ext.to_str())
      .map_or(false, |ext| {
        IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str())
      })
}
