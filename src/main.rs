use std::fs;
mod vec3;

fn main() {
    let width: usize = 200;
    let height: usize = 100;
    let max_color: usize = 255;

    // Begin PPM file header
    let mut buffer = format!("P3\n{} {}\n 255\n", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let r = ((i as f64 / width as f64) * max_color as f64) as u8;
            let g = ((j as f64 / height as f64) * max_color as f64) as u8;
            let b = (0.2 * max_color as f64) as u8;

            buffer = format!("{}{} {} {}\n", buffer, r, g, b);
        }
    }

    match fs::write("output.ppm", buffer) {
        Err(_) => eprintln!("Could not generate the picture!"),
        Ok(_) => ()
    }
}
