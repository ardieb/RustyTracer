use crate::vector::Vec3;
use ndarray::{Array2, array};


#[derive(Debug)]
pub struct Rotation {
    pub alpha: f64, // rotation about z
    pub beta: f64, // rotation about y
    pub gamma: f64, // rotation about x
    mat: Array2<f64>,
}

impl Rotation {
    pub fn new(alpha: f64, beta: f64, gamma: f64) -> Rotation {
        let (cos_alpha, cos_beta, cos_gamma) = (alpha.to_radians().cos(), beta.to_radians().cos(), gamma.to_radians().cos());
        let (sin_alpha, sin_beta, sin_gamma) = (alpha.to_radians().sin(), beta.to_radians().sin(), gamma.to_radians().sin());
        let mat = array![
        [cos_alpha * cos_beta, cos_alpha * sin_beta * sin_gamma - sin_alpha * cos_gamma, cos_alpha * sin_beta * cos_gamma - sin_alpha * sin_gamma],
        [sin_alpha * cos_beta, sin_alpha * sin_beta * sin_gamma + cos_alpha * cos_gamma, sin_alpha * sin_beta * cos_gamma - cos_alpha * sin_gamma],
        [-sin_beta, cos_beta * sin_gamma, cos_beta * cos_gamma],
        ];
        Rotation { alpha, beta, gamma, mat }
    }

    pub fn apply(&self, v: &Vec3) -> Vec3 {
        let v = array![v.x, v.y, v.z];
        let ret = self.mat.t().dot(&v);
        Vec3 {
            x: ret[0],
            y: ret[1],
            z: ret[2],
        }
    }

    pub fn invert(&self, v: &Vec3) -> Vec3 {
        let v = array![v.x, v.y, v.z];
        let ret = self.mat.dot(&v);
        Vec3 {
            x: ret[0],
            y: ret[1],
            z: ret[2],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::rotate::Rotation;
    use crate::vector::Vec3;

    #[test]
    fn test_vecs() {
        let v0 = Vec3::new(0.0, 1.0, 0.0);
        let rot = Rotation::new(90.0, 0.0, 0.0);
        let v1 = rot.apply(&v0);
        let v2 = rot.invert(&v1);
        println!("{:?} {:?} {:?}", v0, v1, v2);
    }
}