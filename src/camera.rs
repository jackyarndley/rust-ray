use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::util::random_in_unit_disk;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
//    w: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = v_fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit();
        let u = (v_up.cross(w)).unit();
        let v = w.cross(u);

        Camera {
            lower_left_corner: look_from - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist,
            horizontal: u * 2.0 * half_width * focus_dist,
            vertical: v * 2.0 * half_height * focus_dist,
            origin: look_from,
            u,
            v,
//            w,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset)
    }
}