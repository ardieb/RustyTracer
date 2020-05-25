use crate::shapes::Shape;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Debug)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Shape for Sphere {
    fn intersect(&self, ray: Ray) -> Option<f64> {
        let displacement = self.position - ray.origin; // Vector from the origin to the sphere center
        let displacement_sqr = displacement.dot(displacement); // The length squared of voc
        let projection_len = displacement.dot(ray.direction); // The length of the projected vector voc into the ray direction

        let a_sqr = displacement_sqr - (projection_len * projection_len); // The length squared of the line between c and the ray
        let r_sqr = self.radius * self.radius; // Radius squared

        // the ray is inside the sphere
        if a_sqr <= r_sqr {
            let b = (r_sqr - a_sqr).sqrt(); // the distance between o and the intersection with the sphere

            let distance = if projection_len - b < 0.0 {
                projection_len + b
            } else {
                projection_len - b
            };

            if distance > 0.0 {
                Some(distance)
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

    fn normal(&self, point: Vec3) -> Vec3 {
        (point - self.position).normalize()
    }
}