use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub mod sphere;
pub mod triangle;

pub use sphere::Sphere;
pub use triangle::Triangle;

pub struct SurfaceInteraction {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

impl SurfaceInteraction {
    pub fn new(t: f64, point: Vec3, normal: Vec3) -> SurfaceInteraction {
        SurfaceInteraction {
            t,
            point,
            normal
        }
    }
}

pub trait Hitable: Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<(SurfaceInteraction, &Material)>;
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
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<(SurfaceInteraction, &Material)> {
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