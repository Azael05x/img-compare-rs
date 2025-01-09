use std::{
  fs,
  path::{Path, PathBuf},
};

/**
* Recursively list all images in the given directory
*/
pub fn list_images_recursively(path: &Path) -> Result<Vec<PathBuf>, String> {
  let mut files = Vec::new();

  fs::read_dir(path)
    .map_err(|err| format!("Could not read directory {}: {}", path.display(), err))?
    .filter_map(Result::ok)
    .for_each(|entry| {
      let path = entry.path();

      if path.is_dir() {
        if let Ok(mut nested_files) = list_images_recursively(&path) {
          files.append(&mut nested_files);
        }
      } else if super::is_image_file::is_image_file(&path) {
        files.push(path);
      }
    });

  Ok(files)
}
