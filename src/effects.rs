use crate::opencv_allowed::{Mat, Error};
use super::filters::conv_filter::ConvFilter;
use super::filters::rank_filter::{get_rank_filter, Mode};
use super::filters::filter::Filter;
use super::kernels::Kernel;

pub fn convolve(src: &Mat, k: Kernel) -> Result<Mat, Error> {
    ConvFilter::new(k).filter(src)
}

pub fn apply_rank_filter(src: &Mat, size: i32, mode: Mode) -> Result<Mat, Error> {
    let rank_filter = get_rank_filter(size, mode);
    rank_filter.filter(src)
}

pub fn erosion(src: &Mat, size: i32, iterations: usize) -> Result<Mat, Error> {
    let mut res = src.clone();
    for _ in 0..iterations {
        res = apply_rank_filter(&res, size, Mode::MinFilter)?;
    }
    Ok(res)
}

pub fn dilation(src: &Mat, size: i32, iterations: usize) -> Result<Mat, Error> {
    let mut res = src.clone();
    for _ in 0..iterations {
        res = apply_rank_filter(&res, size, Mode::MaxFilter)?;
    }
    Ok(res)
}

pub fn closing(src: &Mat, size: i32, iterations: usize) -> Result<Mat, Error> {
    let mut res = src.clone();
    for _ in 0..iterations {
        res = apply_rank_filter(&res, size, Mode::MaxFilter)?;
        res = apply_rank_filter(&res, size, Mode::MinFilter)?;
    }
    Ok(res)
}
pub fn opening(src: &Mat, size: i32, iterations: usize) -> Result<Mat, Error> {
    let mut res = src.clone();
    for _ in 0..iterations {
        res = apply_rank_filter(&res, size, Mode::MinFilter)?;
        res = apply_rank_filter(&res, size, Mode::MaxFilter)?;
    }
    Ok(res)
}
