use logo_lib::opencv_allowed::{imshow, imread, wait_key, Mat};
#[allow(unused_imports)]
use logo_lib::{convert_colors, convolve, erosion, dilation, closing, opening, apply_rank_filter, BGR2HSV, in_range, mask_or};
use logo_lib::{DefaultKernels, Blur, MedianFilter, MinFilter, MaxFilter};
use logo_lib::bounding_box;
use logo_lib::{segment_mask_mut, filter_out_segments, draw_bounding_boxes};

fn main() {
    let filename = "data/7.jpg";
    let image = imread(filename, 1).unwrap();
    let hsv = convert_colors(&image, BGR2HSV).unwrap();

    // let mut blue_mask = in_range(&hsv, (80, 30, 30), (130, 255, 255)).unwrap();
    // imshow("blue_mask", &blue_mask).unwrap();
    // let mut blue_segments = segment_mask_mut(&mut blue_mask).unwrap();
    // blue_segments = filter_out_segments(blue_segments, 7, 5, 60, 60);

    // let red_mask_part1 = in_range(&hsv, (0, 50, 50), (15, 255, 255)).unwrap();
    // let red_mask_part2 = in_range(&hsv, (160, 50, 50), (180, 255, 255)).unwrap();
    // let red_mask = mask_or(&red_mask_part1, &red_mask_part2).unwrap();
    // let mut red_mask_filtered = apply_rank_filter(&red_mask, 3, MedianFilter).unwrap();
    // let mut red_segments = segment_mask_mut(&mut red_mask_filtered).unwrap();
    // red_segments = filter_out_segments(red_segments, 3, 3, 70, 70);

    let mut yellow_mask = in_range(&hsv, (20, 100, 100), (30, 255, 255)).unwrap();
    let mut yellow_mask_filtered = dilation(&mut yellow_mask, 3, 2).unwrap();
    let mut yellow_segments = segment_mask_mut(&mut yellow_mask_filtered).unwrap();
    yellow_segments = filter_out_segments(yellow_segments, 15, 30, 300, 300);

    let test = draw_bounding_boxes(&image, &yellow_segments).unwrap();
    imshow("test", &test).unwrap();
    // imshow("red_mask_filtered", &red_mask_filtered).unwrap();
    imshow("image", &image).unwrap();
    wait_key(0).unwrap();
}