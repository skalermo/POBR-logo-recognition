pub mod opencv_allowed;
mod colors;
mod filters;
mod kernels;
mod effects;
mod primitives;

pub use colors::{convert_colors, Conversion::BGR2HSV, in_range, mask_and, mask_or};
pub use filters::rank_filter::Mode::{MinFilter, MedianFilter, MaxFilter};
pub use effects::{convolve, apply_rank_filter, erosion, dilation, closing, opening};
pub use kernels::DefaultKernels::{self, Blur};
pub use primitives::bounding_box;