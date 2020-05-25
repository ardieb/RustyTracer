use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

use std::fmt::Debug;
use std::marker::Sync;

pub trait Shape: Debug + Sync {
    fn intersect(&self, ray: Ray) -> Option<f64>;
    fn material(&self) -> Material;
    fn normal(&self, hit_point: Vec3) -> Vec3;
}

pub mod sphere;
pub mod plane;
pub mod aabb;