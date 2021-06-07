use logo_lib::opencv_allowed::{imshow, imread, wait_key, Mat};
#[allow(unused_imports)]
use logo_lib::{convert_colors, convolve, apply_rank_filter, BGR2HSV, in_range, mask_or};
use logo_lib::{DefaultKernels, Blur, MedianFilter, MinFilter, MaxFilter};

fn main() {
    let filename = "data/7.jpg";
    let image = imread(filename, 1).unwrap();
    let hsv = convert_colors(&image, BGR2HSV).unwrap();
    // let mut res = Mat::default().unwrap();

    // convolve(&image, &mut res, DefaultKernels::make_kernel_for(Blur)).unwrap();
    // apply_rank_filter(&image, &mut res, 5, MinFilter).unwrap();
    let blue_mask = in_range(&hsv, (80, 50, 50), (130, 255, 255)).unwrap();
    let red_mask = in_range(&hsv, (0, 50, 50), (10, 255, 255)).unwrap();
    let red_mask2 = in_range(&hsv, (160, 50, 50), (180, 255, 255)).unwrap();
    let red = mask_or(&red_mask, &red_mask2).unwrap();
    // let mask = in_range(&hsv, (15, 120, 120), (35, 255, 255)).unwrap();
    imshow("image", &image).unwrap();
    // imshow("res", &res).unwrap();
    imshow("blue_mask", &blue_mask).unwrap();
    imshow("red", &red).unwrap();
    wait_key(0).unwrap();
}