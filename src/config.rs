pub mod constants {
  use std::path::Path;

  /// Supported image extensions
  pub const IMAGE_EXTENSIONS: [&str; 5] = ["jpg", "jpeg", "png", "avif", "webp"];

  pub enum CacheStrategy {
    Disk(&'static Path),
    InMemory,
  }
}
