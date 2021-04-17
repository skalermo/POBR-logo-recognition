pub mod opencv_allowed;
mod colors;

pub use colors::convert_colors;
pub use colors::Conversion::{BGR2HSV};

#[cfg(test)]
#[path = "../tests/tests.rs"]
mod tests;