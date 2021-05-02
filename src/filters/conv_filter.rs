use crate::opencv_allowed::{Mat, MatTrait, Vec3b, Error};
use super::super::kernels::Kernel;
use super::filter::{Filter, clamp_to_u8};

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
