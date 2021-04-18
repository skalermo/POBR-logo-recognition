use crate::kernels::DefaultKernels::Blur;

#[derive(PartialEq, Debug)]
pub struct Kernel {
    pub mat: Vec<Vec<f64>>,
    pub ksize: (i32, i32),
}

impl Kernel {
    fn new(mat: Vec<Vec<i32>>) -> Self {
        if mat.len() == 0 || mat[0].len() == 0 {
            panic!("Kernel takes as a parameter non-empty 2d vector")
        }
        let ksize = (mat.len() as i32, mat[0].len() as i32);
        let mut sum = mat
            .iter()
            .fold(0, |sum, vec| sum + vec.iter().sum::<i32>());
        if sum == 0 {
            sum = 1;
        }
        let mat_ = mat
            .into_iter()
            .map(|vec| vec
                .into_iter()
                .map(|x| x as f64 / sum as f64)
                .collect::<Vec<f64>>())
            .collect::<Vec<Vec<f64>>>();
        Kernel { mat: mat_, ksize }
    }
}

pub enum DefaultKernels {
    Blur,
}

impl DefaultKernels {
    pub fn make_kernel_for(dk: Self) -> Kernel {
        match dk {
            Blur => Kernel::new(
                vec![
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                ]
            ),
        }
    }
}


#[cfg(test)]
mod kernel {
    use super::*;

    #[test]
    #[should_panic]
    fn kernel_new_panics_if_provided_no_rows() {
        Kernel::new(vec![vec![0; 0]; 0]);
    }

    #[test]
    #[should_panic]
    fn kernel_new_panics_if_provided_no_cols() {
        Kernel::new(vec![vec![0; 0]; 1]);
    }

    #[test]
    fn kernel_new() {
        let k = Kernel::new(vec![vec![0, 1, 2]; 2]);
        assert_eq!((2, 3), k.ksize);
        assert_eq!(vec![vec![0., 1. / 6., 2. / 6.]; 2], k.mat);
    }
}

#[cfg(test)]
mod default_kernel {
    use super::*;

    #[test]
    fn default_kernel_blur() {
        let k = Kernel::new(
            vec![
                vec![1, 1, 1],
                vec![1, 1, 1],
                vec![1, 1, 1],
            ]
        );
        assert_eq!(k, DefaultKernels::make_kernel_for(Blur));
    }
}
