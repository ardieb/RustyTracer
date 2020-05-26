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
        let ac = self.c - self.a;
        let ab = self.b - self.a;
        let bc = self.c - self.b;
        let normal = ab.cross(bc).normalize();

        let denom = ray.direction.dot(normal);
        if denom.abs() > crate::EPSILON {
            let t = (self.a - ray.origin).dot(normal) / denom;
            let p = ray.origin + ray.direction * t;
            if ab.cross(p - self.a).dot(normal) < 0.0 &&
                bc.cross(p - self.b).dot(normal) < 0.0 &&
                ac.cross(p - self.c).dot(normal) < 0.0 {
                Some(t)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, _hit_point: Vec3) -> Vec3 {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        -ab.cross(ac).normalize()
    }
}