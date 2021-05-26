use logo_lib::opencv_allowed::{imshow, imread, wait_key, Mat};
#[allow(unused_imports)]
use logo_lib::{convert_colors, convolve, apply_rank_filter, BGR2HSV, in_range};
use logo_lib::{DefaultKernels, Blur, MedianFilter, MinFilter, MaxFilter};

fn main() {
    let filename = "data/Lena.png";
    let image = imread(filename, 1).unwrap();
    // let res = convert_colors(&image, BGR2HSV).unwrap();
    // let mut res = Mat::default().unwrap();

    // convolve(&image, &mut res, DefaultKernels::make_kernel_for(Blur)).unwrap();
    // apply_rank_filter(&image, &mut res, 5, MinFilter).unwrap();
    let mask = in_range(&image, (100, 100, 100), (200, 200, 200)).unwrap();
    imshow("image", &image).unwrap();
    // imshow("res", &res).unwrap();
    imshow("mask", &mask).unwrap();
    wait_key(0).unwrap();
}