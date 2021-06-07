use crate::opencv_allowed::{Mat, Error};
use super::filters::conv_filter::ConvFilter;
use super::filters::rank_filter::{get_rank_filter, Mode};
use super::filters::filter::Filter;
use super::kernels::Kernel;

pub fn convolve(src: &Mat, dst: &mut Mat, k: Kernel) -> Result<(), Error> {
    ConvFilter::new(k).filter(src, dst)
}

pub fn apply_rank_filter(src: &Mat, dst: &mut Mat, size: i32, mode: Mode) -> Result<(), Error> {
    let rank_filter = get_rank_filter(size, mode);
    rank_filter.filter(src, dst)
}