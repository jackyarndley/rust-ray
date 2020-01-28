use rust_ray::vec3::Vec3;
use rust_ray::ray::Ray;
use rust_ray::camera::Camera;
use rust_ray::objects::Hitable;
use rust_ray::util::clamp;
use rust_ray::util::scenes::{random_scene2, simple_scene};

use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Instant;

struct Sample<'a> {
    ray: Option<Ray>,
    color: Vec3,
    depth: usize,
    max_depth: usize,
    world: &'a dyn Hitable
}

impl<'a> Sample<'a> {
    pub fn new(ray: Ray, max_depth: usize, world: &'a dyn Hitable) -> Self {
        Sample {
            ray: Some(ray),
            color: Vec3::new(1.0, 1.0, 1.0),
            depth: 0,
            max_depth,
            world
        }
    }
}

impl<'a> Iterator for Sample<'a> {
    type Item = Vec3;

    // Propagates the ray through the scene to get the color of the sample
    fn next(&mut self) -> Option<Vec3> {
        if self.depth < self.max_depth {
            match self.ray {
                Some(ray) => {
                    // t_min here is set to 0.001 to prevent some shadowing errors
                    match self.world.hit(ray, 0.001, std::f64::INFINITY) {
                        Some((surface_interaction, material)) => {
                            match material.scatter(ray, surface_interaction.normal, surface_interaction.point) {
                                (attenuation, None) => {
                                    self.color *= attenuation;
                                    self.ray = None;
                                }
                                (attenuation, Some(scattered)) => {
                                    self.color *= attenuation;
                                    self.ray = Some(scattered);
                                }
                            }
                        }
                        None => {
                            let unit_direction: Vec3 = ray.direction.unit();
                            let t: f64 = 0.5 * (unit_direction.y + 1.0);
                            self.color *= Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
                            self.ray = None;
                        }
                    }
                    Some(self.color)
                }
                None => {
                    None
                }
            }
        } else {
            None
        }
    }
}

fn main() {
    let width: usize = 300;
    let height: usize = 125;
    let max_color: f64 = 255.999;
    let samples = 64;
    let time = Instant::now();

    print!("Building environment...");
    let world = simple_scene();

    let look_from = Vec3::new(16.0, 2.0, 4.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.2;

    let camera = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 15.0, width as f64 / height as f64, aperture, dist_to_focus);

    println!(" {} objects, {}ms", world.list.len(), time.elapsed().as_millis());
    let time = Instant::now();

    print!("Sampling rays...");
    let pixels = (0..height)
        .into_par_iter()
        .rev()
        .map(|j| {
            let mut part = Vec::with_capacity(width * 3);
            for i in 0..width {
                let mut rng = thread_rng();
                let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);

                for _s in 0..samples {
                    let u: f64 = (i as f64 + rng.gen::<f64>()) / width as f64;
                    let v: f64 = (j as f64 + rng.gen::<f64>()) / height as f64;
                    let r: Ray = camera.get_ray(u, v);
                    col += Sample::new(r, 50, &world).last().unwrap();
                }

                col = col / samples as f64;
                col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

                let ir: u8 = (max_color * clamp(col.x, 0.0, 1.0)) as u8;
                let ig: u8 = (max_color * clamp(col.y, 0.0, 1.0)) as u8;
                let ib: u8 = (max_color * clamp(col.z, 0.0, 1.0)) as u8;

                part.push(ir);
                part.push(ig);
                part.push(ib);
            }
            part
        })
        .flatten()
        .collect::<Vec<u8>>();
    println!(" {} rays, {}ms", width * height * samples, time.elapsed().as_millis());

    image::save_buffer("output.png", &pixels, width as u32, height as u32, image::RGB(8)).unwrap();
}
