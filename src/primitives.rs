use crate::opencv_allowed::{Mat, Error, MatTrait, Vec3b};

static BB_COLOR: [u8; 3] = [4, 255, 16];

pub fn horizontal_line(image: &Mat, y: i32, x_start: i32, x_end: i32) -> Result<Mat, Error> {
    assert!(x_start <= x_end);
    let mut res = image.clone();
    for x in x_start..=x_end {
        res.at_2d_mut::<Vec3b>(y, x)?.0 = BB_COLOR
    }
    Ok(res)
}

pub fn horizontal_line_mut(image: &mut Mat, y: i32, x_start: i32, x_end: i32) -> Result<(), Error> {
    assert!(x_start <= x_end);
    for x in x_start..=x_end {
        image.at_2d_mut::<Vec3b>(y, x)?.0 = BB_COLOR
    }
    Ok(())
}

pub fn vertical_line(image: &Mat, x: i32, y_start: i32, y_end: i32) -> Result<Mat, Error> {
    assert!(y_start <= y_end);
    let mut res = image.clone();
    for y in y_start..=y_end {
        res.at_2d_mut::<Vec3b>(y, x)?.0 = BB_COLOR
    }
    Ok(res)
}

pub fn vertical_line_mut(image: &mut Mat, x: i32, y_start: i32, y_end: i32) -> Result<(), Error> {
    assert!(y_start <= y_end);
    for y in y_start..=y_end {
        image.at_2d_mut::<Vec3b>(y, x)?.0 = BB_COLOR
    }
    Ok(())
}

pub fn bounding_box(image: &Mat, top_left: (i32, i32), bottom_right: (i32, i32)) -> Result<Mat, Error> {
    let mut res = image.clone();
    horizontal_line_mut(&mut res, top_left.0, top_left.1, bottom_right.1);
    horizontal_line_mut(&mut res, bottom_right.0, top_left.1, bottom_right.1);
    vertical_line_mut(&mut res, top_left.1, top_left.0, bottom_right.0);
    vertical_line_mut(&mut res, bottom_right.1, top_left.0, bottom_right.0);
    Ok(res)
}

pub fn bounding_boxes(image: &Mat, boxes: Vec<(i32, i32, i32, i32)>) -> Result<Mat, Error> {
    let mut res = image.clone();
    for b in boxes.iter() {
        let top_left = (b.0, b.1);
        let bottom_right = (b.2, b.3);
        horizontal_line_mut(&mut res, top_left.0, top_left.1, bottom_right.1);
        horizontal_line_mut(&mut res, bottom_right.0, top_left.1, bottom_right.1);
        vertical_line_mut(&mut res, top_left.1, top_left.0, bottom_right.0);
        vertical_line_mut(&mut res, bottom_right.1, top_left.0, bottom_right.0);

    }
    Ok(res)
}
