use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;

pub struct HitData {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

impl HitData {
    pub fn new(t: f64, point: Vec3, normal: Vec3) -> HitData {
        HitData {
            t,
            point,
            normal
        }
    }
}

pub trait Hitable: Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<(HitData, &Material)>;
}

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
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<(HitData, &Material)> {
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
                Some((HitData::new(t1, p, n), &self.material))
            } else {
                let t2 = (-b + discriminant.sqrt()) / a;

                if t2 < t_max && t2 > t_min {
                    let p = r.point_at_parameter(t2);
                    let n = (p - self.center) / self.radius;
                    Some((HitData::new(t2, p, n), &self.material))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

pub struct HitableList {
    list: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList {
            list
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<(HitData, &Material)> {
        let mut closest_so_far = t_max;
        let mut res = None;

        for h in self.list.iter() {
            if let Some((hit_record, material)) = h.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                res = Some((hit_record, material));
            }
        }
        res
    }
}