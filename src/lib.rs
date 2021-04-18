pub mod opencv_allowed;
mod colors;
mod filters;
mod kernels;
mod effects;

pub use colors::{convert_colors, Conversion::BGR2HSV};
pub use effects::convolve;
pub use kernels::DefaultKernels::{self, Blur};

#[cfg(test)]
#[path = "../tests/tests.rs"]
mod tests;