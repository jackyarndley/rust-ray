use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Camera {
        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v)
    }
}