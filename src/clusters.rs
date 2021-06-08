use crate::Segment;

#[derive(Debug)]
pub struct Cluster<'a> {
    row_min: i32,
    row_max: i32,
    col_min: i32,
    col_max: i32,
    yellow_segment: &'a Segment,
    blue_segments: Vec<&'a Segment>,
    red_segments: Vec<&'a Segment>,
}

impl <'a> Cluster<'a> {
    pub fn new(
        row_min: i32,
        row_max: i32,
        col_min: i32,
        col_max: i32,
        yellow_segment: &'a Segment,
        blue_segments: Vec<&'a Segment>,
        red_segments: Vec<&'a Segment>,
    ) -> Cluster<'a> {
        Cluster {
            row_min,
            row_max,
            col_min,
            col_max,
            yellow_segment,
            blue_segments,
            red_segments,
        }
    }

    pub fn get_row_min(&self) -> i32 {
        self.row_min
    }

    pub fn get_col_min(&self) -> i32 {
        self.col_min
    }

    pub fn get_row_max(&self) -> i32 {
        self.row_max
    }

    pub fn get_col_max(&self) -> i32 {
        self.col_max
    }
}
