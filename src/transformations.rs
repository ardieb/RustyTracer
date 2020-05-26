use arrayfire::{Array, Dim4, MatProp};

use crate::vector::Vec3;

use std::fmt;
use std::fmt::{Debug, Formatter};

pub trait Transformation {
    fn transform(&self, _v: &Vec3) -> Vec3 {
        unimplemented!();
    }

    fn inv_transform(&self, _v: &Vec3) -> Vec3 {
        unimplemented!();
    }
}

pub struct Rotation {
    pub alpha: f64, // rotation about x
    pub beta: f64, // rotation about y
    pub gamma: f64, // rotation about z
    mat: Array<f64>,
}

impl Debug for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("alpha", &self.alpha)
            .field("beta", &self.beta)
            .field("gamma", &self.gamma)
            .finish()
    }
}

impl Transformation for Rotation {
    fn transform(&self, v: &Vec3) -> Vec3 {
        let arr = Array::new(&[v.x, v.y, v.z], Dim4::new(&[3, 1, 1, 1]));
        let transformed = arrayfire::matmul(
            &self.mat, &arr, MatProp::TRANS, MatProp::NONE);
        let mut buffer = Vec::<f64>::new();
        buffer.resize(transformed.elements(), 0.0);
        transformed.host(&mut buffer);
        Vec3::from_slice(buffer.as_slice())
    }

    fn inv_transform(&self, v: &Vec3) -> Vec3 {
        let arr = Array::new(&[v.x, v.y, v.z], Dim4::new(&[3, 1, 1, 1]));
        let transformed = arrayfire::matmul(
            &self.mat, &arr, MatProp::NONE, MatProp::NONE);
        let mut buffer = Vec::<f64>::new();
        buffer.resize(transformed.elements(), 0.0);
        transformed.host(&mut buffer);
        Vec3::from_slice(buffer.as_slice())
    }
}

impl Rotation {
    pub fn new(alpha: f64, beta: f64, gamma: f64) -> Rotation {
        let (cos_alpha, cos_beta, cos_gamma) = (f64::cos(alpha), f64::cos(beta), f64::cos(gamma));
        let (sin_alpha, sin_beta, sin_gamma) = (f64::sin(alpha), f64::sin(beta), f64::sin(gamma));

        let mat = Array::new(&[
            cos_alpha*sin_beta, cos_alpha*sin_beta*sin_gamma - sin_alpha*sin_gamma, cos_alpha*sin_beta*cos_gamma + sin_alpha*sin_beta,
            sin_alpha*cos_beta, sin_alpha*sin_beta*sin_gamma + cos_alpha*cos_gamma, sin_alpha*sin_beta*cos_gamma - cos_alpha*sin_gamma,
            -sin_gamma, cos_beta*sin_gamma, cos_beta*cos_gamma,
        ], Dim4::new(&[3, 3, 1, 1]));
        Rotation { alpha, beta, gamma, mat }
    }
}

