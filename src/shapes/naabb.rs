use crate::shapes::Shape;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;
use crate::rotate::Rotation;

#[derive(Debug)]
pub struct Naabb {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
    pub rotation: Rotation,
}

impl Shape for Naabb {
    fn intersect(&self, world_ray: Ray) -> Option<f64> {

        let ray = Ray {
            origin: self.rotation.apply(&world_ray.origin),
            direction: self.rotation.apply(&world_ray.direction),
        };

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

        tmin = tmin.max(ta.x.max(ta.y.max(ta.z)));
        tmax = tmax.min(tb.x.min(tb.y.min(tb.z)));

        if tmin < tmax && tmax > 0.0 {
            Some(tmin)
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, world_hit_point: Vec3) -> Vec3 {
        let hit_point = self.rotation.apply(&world_hit_point);

        let extent = (self.max - self.min) / 2.0;
        let center = (self.min + self.max) / 2.0;
        let local = hit_point - center;

        let mut min = f64::INFINITY;
        let mut ret = Vec3::zero();
        let mut dist = (extent.x - local.x.abs()).abs();

        if dist < min {
            min = dist;
            ret.x = local.x.signum();
        }

        dist = (extent.y - local.y.abs()).abs();
        if dist < min {
            min = dist;
            ret.x = 0.0;
            ret.y = local.y.signum();
        }

        dist = (extent.z - local.z.abs()).abs();
        if dist < min {
            ret.y = 0.0;
            ret.z = local.z.signum();
        }

        self.rotation.invert(&ret)
    }
}