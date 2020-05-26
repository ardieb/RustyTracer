use rusty_tracer::camera::Camera;
use rusty_tracer::color::Color;
use rusty_tracer::light::Light;
use rusty_tracer::light::LightType;
use rusty_tracer::material::Material;
use rusty_tracer::cfg::Cfg;
use rusty_tracer::renderer::Renderer;
use rusty_tracer::vector::Vec3;
use rusty_tracer::shapes::sphere::Sphere;
use rusty_tracer::shapes::plane::Plane;
use rusty_tracer::shapes::aabb::Aabb;
use rusty_tracer::shapes::naabb::Naabb;
use rusty_tracer::rotate::Rotation;

use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use std::time::Instant;

fn display(width: u32, height: u32, buffer: Vec<u32>) {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(width as f64 / 2.0, height as f64 / 2.0);
        WindowBuilder::new()
            .with_title("Rusty Tracer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut hidpi_factor = window.scale_factor();

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(width, height, surface);
        Pixels::new(width, height, surface_texture).unwrap()
    };
    let mut frame = pixels.get_frame();
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let c = buffer[i];
        pixel.copy_from_slice(&[((c >> 16) & 255) as u8, ((c >> 8) & 255) as u8, (c & 255) as u8, 255]);
    }

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if !pixels.render().is_ok() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        // Handle input events
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                hidpi_factor = factor;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}


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
            Vec3::new(0., 10., 10.),
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

    //let buf: Vec<u8> = renderer.render();

    //match image::save_buffer("result.png", buf.as_slice(), width, height, image::ColorType::Rgb8) {
    //    Ok(()) => println!("Saved image to result.png"),
    //    Err(err) => println!("Failed to save image. Encounter error {}", err),
    //}

    //renderer.render_to_file("result.png".to_string());
    let buf: Vec<u32> = renderer.render();
    let duration = now.elapsed();
    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );
    display(width, height, buf);
}
