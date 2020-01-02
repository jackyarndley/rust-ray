use std::fs;

mod vec3;
mod ray;
mod hitable;
mod camera;

use vec3::Vec3;
use ray::Ray;
use camera::Camera;
use hitable::Hitable;
use crate::hitable::{Sphere, HitableList};
use rand::{thread_rng, Rng};

fn color(r: Ray, world: &dyn Hitable) -> Vec3 {
    match world.hit(r, 0.001, std::f64::INFINITY) {
        Some(hit_record) => {
            let target = hit_record.p + hit_record.normal + Vec3::random_in_unit_sphere(thread_rng());
            color(Ray::new(hit_record.p, target - hit_record.p), world) * 0.5
        }
        None => {
            let unit_direction: Vec3 = r.direction.unit();
            let t: f64 = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let width: usize = 800;
    let height: usize = 400;
    let max_color: f64 = 255.999;
    let samples = 100;

    // Begin PPM file header
    let mut buffer = format!("P3\n{} {}\n 255\n", width, height);

    let mut list: Vec<Box<dyn Hitable>> = vec![];

    let mut rng = thread_rng();

    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let world = HitableList::new(list);

    let cam = Camera::new(Vec3::new(-2.0, -1.0, -1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, 0.0));

    for j in (0..height).rev() {
        for i in 0..width {
            let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);

            for _s in 0..samples {
                let u: f64 = (i as f64 + rng.gen::<f64>()) / width as f64;
                let v: f64 = (j as f64 + rng.gen::<f64>()) / height as f64;
                let r: Ray = cam.get_ray(u, v);
                col += color(r, &world);
            }

            col = col / samples as f64;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            let ir: u8 = (max_color * col.x) as u8;
            let ig: u8 = (max_color * col.y) as u8;
            let ib: u8 = (max_color * col.z) as u8;

            buffer = format!("{}{} {} {}\n", buffer, ir, ig, ib);
        }
    }

    match fs::write("output.ppm", buffer) {
        Err(_) => eprintln!("Could not generate the picture!"),
        Ok(_) => ()
    }
}
