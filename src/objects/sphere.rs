use crate::vec3::Vec3;
use crate::material::Material;
use crate::objects::{Hitable, SurfaceInteraction};
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<(SurfaceInteraction, &Material)> {
        let oc: Vec3 = r.origin - self.center;
        let a: f64 = r.direction.dot(r.direction);
        let b: f64 = oc.dot(r.direction);
        let c: f64 = oc.dot(oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;

        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;

            if t1 < t_max && t1 > t_min {
                let p = r.point_at_parameter(t1);
                let n = (p - self.center) / self.radius;
                Some((SurfaceInteraction::new(t1, p, n), &self.material))
            } else {
                let t2 = (-b + discriminant.sqrt()) / a;

                if t2 < t_max && t2 > t_min {
                    let p = r.point_at_parameter(t2);
                    let n = (p - self.center) / self.radius;
                    Some((SurfaceInteraction::new(t2, p, n), &self.material))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}