use logo_lib::opencv_allowed::{imshow, imread, wait_key};
use logo_lib::{convert_colors, BGR2HSV};

fn main() {
    // assert_eq!([87, 227, 189], bgr2hsv([170, 189, 21]));

    let filename = "data/1.jpg";
    let image = imread(filename, 1).unwrap();
    let res = convert_colors(&image, BGR2HSV).unwrap();
    imshow("image", &image).unwrap();
    imshow("res", &res).unwrap();
    wait_key(0).unwrap();
}