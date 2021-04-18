use crate::opencv_allowed::{Mat, MatTrait, MatExprTrait, Error, Vec3b};
use super::kernels::Kernel;

pub trait Filter {
    fn filter(&self, src: &Mat, dst: &mut Mat) -> Result<(), Error> {
        // make sure src and dst have same size
        if dst.cols() != src.cols() || dst.rows() != src.rows() {
            *dst = Mat::zeros(src.rows(), src.cols(), src.typ()?)?.to_mat()?;
        }
        let (dv, du) = (self.size().0 / 2, self.size().1 / 2);
        for v in dv..src.rows()-dv {
            for u in du..src.cols()-du {
                self.apply(src, dst, (v, u))?;
            }
        }
        Ok(())
    }

    fn apply(&self, src: &Mat, dst: &mut Mat, yx: (i32, i32)) -> Result<(), Error>;

    fn size(&self) -> (i32, i32);
}

pub struct ConvFilter {
    kernel: Kernel,
}

impl ConvFilter {
    pub fn new(kernel: Kernel) -> Self {
        ConvFilter { kernel }
    }
}

impl Filter for ConvFilter {
    fn apply(&self, src: &Mat, dst: &mut Mat, vu: (i32, i32)) -> Result<(), Error> {
        let mut acc_p = [0; 3];
        let (kh, kw) = (self.kernel.ksize.0 / 2, self.kernel.ksize.1 / 2);
        for y in -kh..=kh {
            for x in -kw..=kw {
                let f = self.kernel.mat[(y + kh) as usize][(x + kw) as usize];
                let src_p = src.at_2d::<Vec3b>(vu.0 + y, vu.1 + x)?.0;
                acc_p[0] += (src_p[0] as f64 * f).round() as i32;
                acc_p[1] += (src_p[1] as f64 * f).round() as i32;
                acc_p[2] += (src_p[2] as f64 * f).round() as i32;
            }
        }
        let dst_p = Vec3b::from(
            [clamp_to_u8(acc_p[0]),
                clamp_to_u8(acc_p[1]),
                clamp_to_u8(acc_p[2])]
        );
        *dst.at_2d_mut(vu.0, vu.1)? = dst_p;
        Ok(())
    }

    fn size(&self) -> (i32, i32) {
        self.kernel.ksize
    }
}

fn clamp_to_u8(number: i32) -> u8 {
    match number {
        _ if number > 255 => 255u8,
        _ if number < 0 => 0u8,
        _ => number as u8,
    }
}

#[cfg(test)]
mod clamp_to_u8 {
    use super::clamp_to_u8;

    #[test]
    fn argument_gt_255() {
        assert_eq!(255, clamp_to_u8(256));
    }

    #[test]
    fn argument_lt_0() {
        assert_eq!(0, clamp_to_u8(-1));
    }

    #[test]
    fn argument_is_u8() {
        assert_eq!(100, clamp_to_u8(100));
    }
}
