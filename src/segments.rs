use crate::opencv_allowed::{Mat, MatTrait, MatExprTrait, Vec3b, Error};
use std::collections::VecDeque;
use opencv::core::Vec3;
use crate::primitives::bounding_boxes;


#[derive(Debug)]
pub struct Segment {
    row_min: i32,
    row_max: i32,
    col_min: i32,
    col_max: i32,
    pixel_coords: Vec<(i32, i32)>,
    border_pixel_coords: Vec<(i32, i32)>,
}

impl Segment {
    pub fn get_row_min(&self) -> i32 {
        self.row_min
    }

    pub fn get_row_max(&self) -> i32 {
        self.row_max
    }

    pub fn get_col_min(&self) -> i32 {
        self.col_min
    }

    pub fn get_col_max(&self) -> i32 {
        self.col_max
    }

    pub fn get_width(&self) -> u32 {
        (self.col_max - self.col_min + 1) as u32
    }

    pub fn get_height(&self) -> u32 {
        (self.row_max - self.row_min + 1) as u32
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.col_min < other.col_min && other.col_max < self.col_max &&
            self.row_min < other.row_min && other.row_max < self.row_max
    }
}

static BG_COLOR: [u8; 3] = [0; 3];

pub fn segment_mask_mut(mask: &mut Mat) -> Result<Vec<Segment>, Error> {
    let mut segments = Vec::<Segment>::new();
    for y in 0..mask.rows() {
        for x in 0..mask.cols() {
            if mask.at_2d::<Vec3b>(y, x)?.0 != BG_COLOR {
                if let Ok(seg) = flood_fill_segment(mask, (y, x)) {
                    segments.push(seg);
                }
            }
        }
    }
    Ok(segments)
}

pub fn filter_out_segments(mut segments: Vec<Segment>, min_height: u32, min_width: u32, max_height: u32, max_width: u32) -> Vec<Segment> {
    segments = segments
        .into_iter()
        .filter(|seg| !(
            seg.get_height() < min_height ||
                seg.get_width() < min_width ||
                seg.get_height() > max_height ||
                seg.get_width() > max_width
        ))
        .collect::<Vec<Segment>>();
    segments
}

fn flood_fill_segment(mask: &mut Mat, seed: (i32, i32)) -> Result<Segment, Error> {
    let mut row_min = mask.rows();
    let mut row_max = 0;
    let mut col_min = mask.cols();
    let mut col_max = 0;
    let mut pixel_coords = Vec::<(i32, i32)>::new();
    let mut border_pixel_coords = Vec::<(i32, i32)>::new();
    let mut stack  = VecDeque::<(i32, i32)>::new();

    stack.push_back(seed);
    while !stack.is_empty() {
        let cur_pixel_coords = stack.pop_front().unwrap();
        if let Ok(pixel) = mask.at_2d::<Vec3b>(cur_pixel_coords.0, cur_pixel_coords.1) {
            if pixel.0 != BG_COLOR {
                if row_min > cur_pixel_coords.0 {
                    row_min = cur_pixel_coords.0;
                }
                if row_max < cur_pixel_coords.0 {
                    row_max = cur_pixel_coords.0;
                }
                if col_min > cur_pixel_coords.1 {
                    col_min = cur_pixel_coords.1;
                }
                if col_max < cur_pixel_coords.1 {
                    col_max = cur_pixel_coords.1;
                }
                pixel_coords.push(cur_pixel_coords);
                mask.at_2d_mut::<Vec3b>(cur_pixel_coords.0, cur_pixel_coords.1)?.0 = BG_COLOR;
                if is_border_color(&mask, cur_pixel_coords.0, cur_pixel_coords.1).unwrap_or(false) {
                    border_pixel_coords.push(cur_pixel_coords);
                }
                stack.push_back((cur_pixel_coords.0 - 1, cur_pixel_coords.1));
                stack.push_back((cur_pixel_coords.0 + 1, cur_pixel_coords.1));
                stack.push_back((cur_pixel_coords.0, cur_pixel_coords.1 - 1));
                stack.push_back((cur_pixel_coords.0, cur_pixel_coords.1 + 1));
            }
        };
    }
    Ok(Segment {
        row_min,
        row_max,
        col_min,
        col_max,
        pixel_coords,
        border_pixel_coords,
    })
}

fn is_border_color(mask: &Mat, y: i32, x: i32) -> Result<bool, Error> {
   let res = mask.at_2d::<Vec3b>(y - 1, x)?.0 == BG_COLOR ||
       mask.at_2d::<Vec3b>(y + 1, x)?.0 == BG_COLOR ||
       mask.at_2d::<Vec3b>(y, x - 1)?.0 == BG_COLOR ||
       mask.at_2d::<Vec3b>(y, x + 1)?.0 == BG_COLOR;
   Ok(res)
}