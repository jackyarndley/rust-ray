use std::fs;

mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

fn color(r: Ray) -> Vec3 {
    let unit_direction: Vec3 = r.direction.unit();
    let t: f64 = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let width: usize = 200;
    let height: usize = 100;
    let max_color: f64 = 255.999;

    // Begin PPM file header
    let mut buffer = format!("P3\n{} {}\n 255\n", width, height);

    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..height).rev() {
        for i in 0..width {
            let u: f64 = i as f64 / width as f64;
            let v: f64 = j as f64 / height as f64;

            let r: Ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col: Vec3 = color(r);

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
