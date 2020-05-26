use crate::shapes::Shape;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Debug)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub material: Material,
}

impl Shape for Triangle {
    fn intersect(&self, ray: Ray) -> Option<f64> {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let pvec = ray.direction.cross(ac);
        let det = ab.dot(pvec);
        if det.abs() < crate::EPSILON {
            None
        } else {
            let tvec = ray.origin - self.a;
            let u = tvec.dot(pvec) / det;
            if u < 0.0 || u > 1.0 {
                None
            } else {
                let qvec = tvec.cross(ab);
                let v = ray.direction.dot(qvec) / det;
                if v < 0.0 || u + v > 1.0 {
                    None
                } else {
                    Some(ac.dot(qvec) / det)
                }
            }
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, _hit_point: Vec3) -> Vec3 {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        -ab.cross(ac)
    }
}