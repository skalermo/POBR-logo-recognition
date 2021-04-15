use logo_lib::opencv_allowed::{ imshow, imread, wait_key };

fn main() {
    let filename = "data/1.jpg";
    let image = imread(filename, 1).unwrap();
    imshow("image", &image).unwrap();
    wait_key(0).unwrap();
}