use rust_ray::vec3::Vec3;
use rust_ray::ray::Ray;
use rust_ray::camera::Camera;
use rust_ray::objects::Hitable;
use rust_ray::util::{clamp, random_scene, random_scene2};

use rand::{thread_rng, Rng};
use rayon::prelude::*;

fn color(r: Ray, world: &dyn Hitable, depth: usize) -> Vec3 {
    match world.hit(r, 0.001, std::f64::INFINITY) {
        Some((hit_record, material)) => {
            let (scattered, attenuation, b) = material.scatter(r, hit_record.normal, hit_record.point);

            if depth < 8 && b {
                if b {
                    color(scattered, world, depth + 1) * attenuation
                } else {
                    attenuation
                }
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction: Vec3 = r.direction.unit();
            let t: f64 = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let width: usize = 300;
    let height: usize = 300;
    let max_color: f64 = 255.999;
    let samples = 128;

    let world = random_scene2();

    let look_from = Vec3::new(16.0, 10.0, 4.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.0;

    let camera = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 25.0, width as f64 / height as f64, aperture, dist_to_focus);

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
                    col += color(r, &world, 0);
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

    image::save_buffer("output.png", &pixels, width as u32, height as u32, image::RGB(8)).unwrap();
}
