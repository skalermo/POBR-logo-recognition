use crate::opencv_allowed::{Mat, Error};
use super::filters::{ConvFilter, Filter};
use super::kernels::Kernel;

pub fn convolve(src: &Mat, dst: &mut Mat, k: Kernel) -> Result<(), Error>{
    Ok(ConvFilter::new(k).filter(src, dst)?)
}
