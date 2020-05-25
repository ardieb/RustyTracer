#![allow(dead_code)]

pub mod camera;
pub mod color;
pub mod intersectable;
pub mod light;
pub mod material;
pub mod cfg;
pub mod ray;
pub mod renderer;
pub mod vector;

const EPSILON: f64 = 1e-6;