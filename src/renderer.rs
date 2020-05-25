use crate::camera::Camera;
use crate::color::Color;
use crate::intersectable::Intersectable;
use crate::light::Light;
use crate::cfg::Cfg;
use crate::ray::Ray;
use rayon::prelude::*;
use image::ImageBuffer;

#[derive(Debug)]
pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub objects: Vec<Box<dyn Intersectable>>,
    pub lights: Vec<Light>,
    pub bg_color: Color,
    pub options: Cfg,
}

impl Renderer {
    pub fn render_to_file(&self, filename: String) {
        let mut imgbuf = ImageBuffer::new(self.width, self.height);

        let gamma_correction = self.options.gamma.recip();

        let w = f64::from(self.width);
        let h = f64::from(self.height);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let u = f64::from(x) / w;
            let v = f64::from(y) / h;

            let ray = self.camera.get_ray(u, v);

            let color = Ray::cast_ray(ray, &self.objects, &self.lights, &self.options, 0)
                .unwrap_or(self.bg_color);

            *pixel = color.gamma_rgb(gamma_correction);
        }

        imgbuf.save(filename).unwrap();
    }

    pub fn render_to_buf(&self) -> Vec<u32> {
        let w = f64::from(self.width);
        let h = f64::from(self.height);
        let gamma_correction = self.options.gamma.recip();
        (0..self.width * self.height)
            .into_par_iter()
            .map(|pixel| {
                let x = pixel % self.height;
                let y = pixel / self.height;

                let u = f64::from(x) / w;
                let v = f64::from(y) / h;

                let ray = self.camera.get_ray(u, v);

                let color = Ray::cast_ray(ray, &self.objects, &self.lights, &self.options, 0)
                    .unwrap_or(self.bg_color);
                color.to_u32(gamma_correction)
            })
            .collect()
    }
}