use crate::shapes::Shape;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Shape for Aabb {
    fn intersect(&self, ray: Ray) -> Option<f64> {
        let mut tmin = -f64::INFINITY;
        let mut tmax = f64::INFINITY;
        let inv_dir = 1.0 / ray.direction;
        let t0s = (self.min - ray.origin) * inv_dir;
        let t1s = (self.max - ray.origin) * inv_dir;
        let ta = Vec3::new(
            t0s.x.min(t1s.x),
            t0s.y.min(t1s.y),
            t0s.z.min(t1s.z),
        );
        let tb = Vec3::new(
            t0s.x.max(t1s.x),
            t0s.y.max(t1s.y),
            t0s.z.max(t1s.z),
        );

        let tmin = tmin.max(ta.x.max(ta.y.max(ta.z)));
        let tmax = tmax.min(tb.x.min(tb.y.min(tb.z)));

        if tmin < tmax && tmax > 0.0 {
            Some(tmin)
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, hit_point: Vec3) -> Vec3 {
        let extent = (self.max - self.min) / 2.0;
        let center = (self.min + self.max) / 2.0;
        let local = hit_point - center;

        let mut min = f64::INFINITY;
        let mut ret = Vec3::zero();
        let mut dist = (extent.x - local.x.abs()).abs();

        if dist < min {
            min = dist;
            ret = Vec3 { x: local.x.signum(), y: 0.0, z: 0.0 };
        }

        dist = (extent.y - local.y.abs()).abs();
        if dist < min {
            min = dist;
            ret = Vec3 { x: 0.0, y: local.y.signum(), z: 0.0 };
        }

        dist = (extent.z - local.z.abs()).abs();
        if dist < min {
            min = dist;
            ret = Vec3 { x: 0.0, y: 0.0, z: local.y.signum() };
        }

        ret
    }
}
