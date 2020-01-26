use crate::vec3::Vec3;
use crate::material::Material;
use crate::objects::{Hitable, SurfaceInteraction};
use crate::ray::Ray;

pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    normal: Vec3,
    material: Material
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Self {
        let normal = (v1 - v0).cross(v2 - v0).unit();
        Triangle {
            v0,
            v1,
            v2,
            normal,
            material
        }
    }

    pub fn new2(v0: Vec3, v1: Vec3, v2: Vec3, normal: Vec3, material: Material) -> Self {
        Triangle {
            v0,
            v1,
            v2,
            normal,
            material
        }
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<(SurfaceInteraction, &Material)> {
        let eps = 0.0000001;

        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = r.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -eps && a < eps {
            return None
        }

        let f = 1.0 / a;
        let s = r.origin - self.v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None
        }

        let q = s.cross(edge1);
        let v = f * r.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None
        }

        let t = f * edge2.dot(q);

        if t > t_min && t < t_max {
            let plane_hit = r.point_at_parameter(t);
            Some((SurfaceInteraction::new(t, plane_hit, self.normal), &self.material))
        } else {
            None
        }
    }
}