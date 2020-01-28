use rust_ray::util::{clamp, Ray, Vec3, simple_scene, Sample};

use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let width: usize = 2400;
    let height: usize = 800;
    let max_color: f64 = 255.999;
    let samples = 128;
    let time = Instant::now();

    print!("Building environment...");
    let (camera, world) = simple_scene(width, height);

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
