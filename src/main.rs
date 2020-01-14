mod vec3;
mod ray;
mod hitable;
mod camera;
mod material;

use vec3::Vec3;
use ray::Ray;
use camera::Camera;
use hitable::Hitable;
use crate::hitable::{Sphere, HitableList};
use rand::{thread_rng, Rng};
use crate::material::Material;
use rand::prelude::ThreadRng;
use rayon::prelude::*;

fn color(r: Ray, world: &dyn Hitable, depth: usize) -> Vec3 {
    match world.hit(r, 0.001, std::f64::INFINITY) {
        Some((hit_record, material)) => {
            let (scattered, attenuation, b) = material.scatter(r, hit_record.normal, hit_record.p);
            if depth < 50 && b {
                color(scattered, world, depth + 1) * attenuation
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

fn random_scene(mut rng: ThreadRng) -> HitableList {
    let mut list: Vec<Box<dyn Hitable>> = vec![];
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian {attenuation: Vec3::new(0.5, 0.5, 0.5)})));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    list.push(Box::new(Sphere::new(center, 0.2, Material::Lambertian {attenuation: Vec3::new(rng.gen::<f64>() * rng.gen::<f64>(), rng.gen::<f64>() * rng.gen::<f64>(), rng.gen::<f64>() * rng.gen::<f64>())})));
                } else if choose_mat < 0.95 {
                    list.push(Box::new(Sphere::new(center, 0.2, Material::Metal {attenuation: Vec3::new(0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>())), fuzziness: 0.5 * rng.gen::<f64>()})));
                } else {
                    list.push(Box::new(Sphere::new(center, 0.2, Material::Dielectric {refraction: 1.5})));
                }
            }
        }
    }

    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::Dielectric {refraction: 1.5})));
    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Material::Lambertian {attenuation: Vec3::new(0.4, 0.2, 0.1)})));
    list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Material::Metal {attenuation: Vec3::new(0.7, 0.6, 0.5), fuzziness: 0.0})));

    HitableList::new(list)
}

fn main() {
    let width: usize = 1200;
    let height: usize = 500;
    let max_color: f64 = 255.999;
    let samples = 100;

    let world = random_scene(thread_rng());

    let look_from = Vec3::new(16.0, 2.0, 4.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.2;

    let cam = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 15.0, width as f64 / height as f64, aperture, dist_to_focus);

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
                    let r: Ray = cam.get_ray(u, v);
                    col += color(r, &world, 0);
                }

                col = col / samples as f64;
                col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

                let ir: u8 = (max_color * col.x) as u8;
                let ig: u8 = (max_color * col.y) as u8;
                let ib: u8 = (max_color * col.z) as u8;

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
