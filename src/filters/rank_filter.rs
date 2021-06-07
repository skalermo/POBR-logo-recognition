use crate::opencv_allowed::{Mat, MatTrait, Vec3b, Error};
use super::filter::{Filter};

pub trait RankFilter {
    fn choose_one_from(&self, pixels: &Vec<u8>) -> u8;
    fn size(&self) -> i32;
}

struct MinFilter {
    size: i32,
}

impl RankFilter for MinFilter {
    fn choose_one_from(&self, pixels: &Vec<u8>) -> u8 {
        *pixels.iter().min().unwrap()
    }

    fn size(&self) -> i32 {
        self.size
    }
}

struct MedianFilter {
    size: i32,
}

impl RankFilter for MedianFilter {
    fn choose_one_from(&self, pixels: &Vec<u8>) -> u8 {
        let n = self.size * self.size;
        let idx = n / 2 + 1;
        let mut sorted = pixels.clone();
        sorted.sort();
        sorted.into_iter().nth(idx as usize).unwrap()
    }

    fn size(&self) -> i32 {
        self.size
    }
}

struct MaxFilter {
    size: i32,
}

impl RankFilter for MaxFilter {
    fn choose_one_from(&self, pixels: &Vec<u8>) -> u8 {
        *pixels.into_iter().max().unwrap()
    }

    fn size(&self) -> i32 {
        self.size
    }
}

pub enum Mode {
    MinFilter,
    MedianFilter,
    MaxFilter,
}

pub fn get_rank_filter(size: i32, mode: Mode) -> Box<dyn Filter> {
    match mode {
        Mode::MinFilter => Box::new(MinFilter{size}),
        Mode::MedianFilter => Box::new(MedianFilter{size}),
        Mode::MaxFilter => Box::new(MaxFilter{size}),
    }
}

impl<T> Filter for T
where
    T: RankFilter
{
    fn apply(&self, src: &Mat, dst: &mut Mat, vu: (i32, i32)) -> Result<(), Error> {
        let (kh, kw) = (self.size() / 2, self.size() / 2);
        let mut pixels = vec![vec![0; (self.size() * self.size()) as usize]; 3];

        for y in -kh..=kh {
            for x in -kw..=kw {
                let p = src.at_2d::<Vec3b>(vu.0 + y, vu.1 + x)?.0;
                pixels[0][((y+kh) * self.size() + x+kw) as usize] = p[0];
                pixels[1][((y+kh) * self.size() + x+kw) as usize] = p[1];
                pixels[2][((y+kh) * self.size() + x+kw) as usize] = p[2];
            }
        }

        let mut dst_p = dst.at_2d_mut::<Vec3b>(vu.0, vu.1)?;
        dst_p.0[0] = self.choose_one_from(&pixels[0]);
        dst_p.0[1] = self.choose_one_from(&pixels[1]);
        dst_p.0[2] = self.choose_one_from(&pixels[2]);

        Ok(())
    }

    fn size(&self) -> (i32, i32) {
        (self.size(), self.size())
    }
}
