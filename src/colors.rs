use crate::opencv_allowed::{Mat, MatTrait, MatExprTrait, Vec3b, Error};

pub enum Conversion {
    BGR2HSV,
}

pub fn convert_colors(image: &Mat, conversion: Conversion) -> Result<Mat, Error> {
    let mut res = Mat::zeros(image.rows(), image.cols(), image.typ()?)?.to_mat()?;

    for y in 0..image.rows() {
        for x in 0..image.cols() {
            let convert = match conversion {
                Conversion::BGR2HSV => bgr2hsv,
            };
            let pixel = convert(image.at_2d::<Vec3b>(y, x)?.0);
            *res.at_2d_mut::<Vec3b>(y, x)? = Vec3b::from(pixel);
        }
    }

    Ok(res)
}

fn bgr2hsv(pixel: [u8; 3]) -> [u8; 3] {
    // [0, 0, 255] -> [0, 255, 255]

    let r = pixel[2] as i32;
    let g = pixel[1] as i32;
    let b = pixel[0] as i32;
    let min = *pixel.iter().min().unwrap() as i32;
    let max = *pixel.iter().max().unwrap() as i32;
    let v = max;
    let d = max - min;
    let s = if v == 0 {0} else {(d as f32 * 255. / v as f32).round() as u8};

    let h;
    if max == min {
        h = 0.;
    } else {
        h = match max {
            _ if max == r => (g - b) as f32 / d as f32 + (if g<b {6.} else {0.}),
            _ if max == g => (b - r) as f32 / d as f32 + 2.,
            _ if max == b => (r - g) as f32 / d as f32 + 4.,
            _ => unreachable!(),
        } / 6.;
    }

    [(h * 180.).round() as u8, s as u8, v as u8]
}

pub fn in_range(image: &Mat, low: (u8, u8, u8), high: (u8, u8, u8)) -> Result<Mat, Error> {
    let mut mask = Mat::zeros(image.rows(), image.cols(), image.typ()?)?.to_mat()?;
    for y in 0..image.rows() {
        for x in 0..image.cols() {
            if in_range_inner(image.at_2d::<Vec3b>(y, x)?.0, low, high) {
                mask.at_2d_mut::<Vec3b>(y, x)?.0 = [255u8; 3];
            }
        }
    }
    Ok(mask)
}

fn in_range_inner(pixel: [u8; 3], low: (u8, u8, u8), high: (u8, u8, u8)) -> bool {
    pixel[0] >= low.0 && pixel[0] <= high.0 &&
        pixel[1] >= low.1 && pixel[1] <= high.1 &&
        pixel[2] >= low.2 && pixel[2] <= high.2
}

pub fn mask_and(mask1: &Mat, mask2: &Mat) -> Result<Mat, Error> {
    assert!(mask1.rows() == mask2.rows() && mask1.cols() == mask2.cols());
    let zeros = [0, 0, 0];
    let mut mask = Mat::zeros(mask1.rows(), mask1.cols(), mask1.typ()?)?.to_mat()?;
    for y in 0..mask.rows() {
        for x in 0..mask.cols() {
            if mask1.at_2d::<Vec3b>(y, x)?.0 != zeros && mask2.at_2d::<Vec3b>(y, x)?.0 != zeros {
                mask.at_2d_mut::<Vec3b>(y, x)?.0 = [255u8; 3];
            }
        }
    }
    Ok(mask)
}

pub fn mask_or(mask1: &Mat, mask2: &Mat) -> Result<Mat, Error> {
    assert!(mask1.rows() == mask2.rows() && mask1.cols() == mask2.cols());
    let zeros = [0, 0, 0];
    let mut mask = Mat::zeros(mask1.rows(), mask1.cols(), mask1.typ()?)?.to_mat()?;
    for y in 0..mask.rows() {
        for x in 0..mask.cols() {
            if mask1.at_2d::<Vec3b>(y, x)?.0 != zeros || mask2.at_2d::<Vec3b>(y, x)?.0 != zeros {
                mask.at_2d_mut::<Vec3b>(y, x)?.0 = [255u8; 3];
            }
        }
    }
    Ok(mask)
}

#[cfg(test)]
mod test_color_conversions {
    use super::*;

    #[test]
    fn test_bgr2hsv_1() {
        assert_eq!([87, 227, 189], bgr2hsv([170, 189, 21]));
    }

    #[test]
    fn test_bgr2hsv_2() {
        assert_eq!([56, 208, 242], bgr2hsv([45, 242, 74]));
    }

    #[test]
    fn test_bgr2hsv_3() {
        assert_eq!([9, 200, 153], bgr2hsv([33, 70, 153]));
    }

    #[test]
    fn test_bgr2hsv_4() {
        assert_eq!([21, 231, 237], bgr2hsv([22, 169, 237]));
    }

    #[test]
    fn test_bgr2hsv_5() {
        assert_eq!([142, 245, 226], bgr2hsv([226, 9, 168]));
    }
}

#[cfg(test)]
mod test_in_range {
    use super::*;

    #[test]
    fn test_pixel_is_in_range() {
        assert!(in_range_inner([120, 240, 84], (119, 200, 84), (121, 240, 90)))
    }

    #[test]
    fn test_first_component_not_in_range() {
        assert!(!in_range_inner([120, 240, 84], (121, 200, 84), (121, 240, 90)))
    }

    #[test]
    fn test_second_component_not_in_range() {
        assert!(!in_range_inner([120, 240, 84], (119, 200, 84), (121, 230, 90)))
    }

    #[test]
    fn test_third_component_not_in_range() {
        assert!(!in_range_inner([120, 240, 84], (119, 200, 70), (121, 240, 83)))
    }
}
