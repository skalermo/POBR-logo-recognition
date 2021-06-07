use logo_lib::opencv_allowed::{imshow, imread, wait_key, Mat};
#[allow(unused_imports)]
use logo_lib::{convert_colors, convolve, erosion, dilation, closing, opening, apply_rank_filter, BGR2HSV, in_range, mask_or};
use logo_lib::{DefaultKernels, Blur, MedianFilter, MinFilter, MaxFilter};
use logo_lib::bounding_box;

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
    // let red = mask_or(&red_mask, &red_mask2).unwrap();
    // let filtered_red_mask = erosion(&red, 3, 1).unwrap();
    let filtered_blue_mask = closing(&blue_mask, 3, 1).unwrap();
    let bb_test = bounding_box(&image, (10, 10), (100, 100)).unwrap();
    imshow("image", &image).unwrap();
    imshow("blue", &blue_mask).unwrap();
    imshow("fblue", &filtered_blue_mask).unwrap();
    imshow("bb_test", &bb_test).unwrap();
    // imshow("red", &red).unwrap();
    // imshow("fred", &filtered_red_mask).unwrap();
    wait_key(0).unwrap();
}