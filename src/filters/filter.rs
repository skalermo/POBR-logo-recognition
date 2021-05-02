use crate::opencv_allowed::{Mat, MatTrait, MatExprTrait, Error};

pub trait Filter {
    fn filter(&self, src: &Mat, dst: &mut Mat) -> Result<(), Error> {
        // make sure src and dst have same size
        if dst.cols() != src.cols() || dst.rows() != src.rows() {
            *dst = Mat::zeros(src.rows(), src.cols(), src.typ()?)?.to_mat()?;
        }
        let (dv, du) = (self.size().0 / 2, self.size().1 / 2);
        for v in dv..src.rows() - dv {
            for u in du..src.cols() - du {
                self.apply(src, dst, (v, u))?;
            }
        }
        Ok(())
    }

    fn apply(&self, src: &Mat, dst: &mut Mat, yx: (i32, i32)) -> Result<(), Error>;

    fn size(&self) -> (i32, i32);
}

pub fn clamp_to_u8(number: i32) -> u8 {
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
