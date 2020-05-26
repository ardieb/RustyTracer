use rusty_trace::camera::Camera;
use rusty_trace::color::Color;
use rusty_trace::light::Light;
use rusty_trace::light::LightType;
use rusty_trace::material::Material;
use rusty_trace::cfg::Cfg;
use rusty_trace::renderer::Renderer;
use rusty_trace::vector::Vec3;
use rusty_trace::shapes::sphere::Sphere;
use rusty_trace::shapes::plane::Plane;
use rusty_trace::shapes::aabb::Aabb;
use rusty_trace::shapes::naabb::Naabb;
use rusty_trace::rotate::Rotation;

use image::load_from_memory;

use std::time::Instant;
use std::iter::Iterator;

fn main() {
    let options = Cfg {
        max_rays: 4,
        gamma: 0.85,
        diffuse: true,
        specular: true,
        shadows: true,
        reflections: true,
        opacity: true,
    };

    let width = 1920;
    let height = 1080;
    let aspect_ratio = width as f64 / height as f64;

    let renderer = Renderer {
        width,
        height,
        camera: Camera::new(
            Vec3::new(0., -3., 10.),
            Vec3::new(0., 0., -20.),
            60.,
            aspect_ratio,
            0.,
        ),
        objects: vec![
            Box::new( Naabb {
                min: Vec3::new(-5.0, 0.0, 0.0),
                max: Vec3::new(2.0, 2.0, 2.0),
                material: Material {
                    color: Color::from_u8(0xD4, 0xAF, 0x37),
                    diffuse: 0.8,
                    specular: 0.2,
                    specular_exponent: 5.0,
                    reflectiveness: 0.6,
                    opacity: 1.0,
                },
                rotation: Rotation::new(45.0, 0.0, 0.0),
            }),
            Box::new(Aabb {
                min: Vec3::new(-1., 0., -10.),
                max: Vec3::new(3.0, 3.0, 0.0),
                material: Material {
                    color: Color::from_u8(0xD4, 0xAF, 0x37),
                    diffuse: 0.8,
                    specular: 0.2,
                    specular_exponent: 5.0,
                    reflectiveness: 0.6,
                    opacity: 1.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(3.0, 3.0, -3.0),
                radius: 2.8,
                material: Material {
                    color: Color::from_u8(0xff, 0x55, 0x55),
                    diffuse: 0.6,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 0.0,
                    opacity: 1.0
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(-10.0, -5.0, -16.0),
                radius: 2.8,
                material: Material {
                    color: Color::from_u8(0xff, 0x55, 0x55),
                    diffuse: 0.6,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 0.0,
                    opacity: 1.0
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(10.0, -5.0, -13.0),
                radius: 2.0,
                material: Material {
                    color: Color::from_u8(0x40, 0xe0, 0xd0),
                    diffuse: 0.6,
                    specular: 5.0,
                    specular_exponent: 500.0,
                    reflectiveness: 0.0,
                    opacity: 1.0
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(3.0, -5.0, -20.0),
                radius: 2.8,
                material: Material {
                    color: Color::from_u8(0x77, 0xbb, 0x77),
                    diffuse: 0.5,
                    specular: 0.2,
                    specular_exponent: 2.0,
                    reflectiveness: 0.0,
                    opacity: 1.0
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(5.0, -4.0, -30.0),
                radius: 3.0,
                material: Material {
                    color: Color::from_u8(0x2f, 0x8d, 0xff),
                    diffuse: 0.6,
                    specular: 3.0,
                    specular_exponent: 50.0,
                    reflectiveness: 0.0,
                    opacity: 1.0
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(-10.0, 5.0, -20.0),
                radius: 5.0,
                material: Material {
                    color: Color::new(0.1, 0.1, 0.1),
                    diffuse: 0.0,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                    opacity: 1.0
                },
            }),
            Box::new(Plane {
                position: Vec3::new(0.0, -8.0, 0.0),
                normal: Vec3::new(0.0, -1.0, 0.0),
                material: Material {
                    color: Color::from_u8(0xD4, 0xAF, 0x37),
                    diffuse: 0.8,
                    specular: 0.2,
                    specular_exponent: 5.0,
                    reflectiveness: 0.6,
                    opacity: 1.0
                },
            }),
        ],
        lights: vec![
            Light {
                light_type: LightType::Point,
                position: Vec3::new(-40.0, 20.0, 20.0),
                intensity: 1.0,
                color: Color::white(),
            },
            Light {
                light_type: LightType::Point,
                position: Vec3::new(40.0, 20.0, 20.0),
                intensity: 0.8,
                color: Color::new(0.66, 0.0, 0.66),
            },
            Light {
                light_type: LightType::Point,
                position: Vec3::new(00.0, 50.0, 0.0),
                intensity: 0.8,
                color: Color::from_u8(0xa6, 0x7c, 0x00),
            },
            Light {
                light_type: LightType::Ambient,
                position: Vec3::zero(),
                intensity: 0.25,
                color: Color::white(),
            },
        ],
        bg_color: Color::black(),
        options,
    };

    let now = Instant::now();
    use std::iter::once;

    let buf: Vec<u8> = renderer.render().iter().flat_map(|c| {
        let b = (c >> 0) as u8;
        let g = (c >> 16) as u8;
        let r = (c >> 24) as u8;
        once(r).chain(once(g).chain(once(b)))
    }).collect();

    if let Ok(image) = load_from_memory(buf.as_slice()) {
        image.save("result.png".to_string());
    } else {
        panic!("Failed to load from buffer");
    }
    //renderer.render_to_file("result.png".to_string());
    let duration = now.elapsed();

    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );
}
