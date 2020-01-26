use crate::vec3::Vec3;
use crate::material::Material;
use crate::objects::{Hitable, SurfaceInteraction};
use crate::ray::Ray;

pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    d:  f64,
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
            d: normal.dot(v0),
            normal,
            material
        }
    }

    pub fn new2(v0: Vec3, v1: Vec3, v2: Vec3, normal: Vec3, material: Material) -> Self {
        Triangle {
            v0,
            v1,
            v2,
            d: normal.dot(v0),
            normal,
            material
        }
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<(SurfaceInteraction, &Material)> {
        let normal_dot_direction = self.normal.dot(r.direction);

        if normal_dot_direction.abs() < 0.00001 {
            return None
        }

        // Check for intersection with the triangle plane
        let t = -(self.normal.dot(r.origin) + self.d) / normal_dot_direction;
//        println!("{}", t);

        if t < t_min {
            return None
        }

        let plane_hit = r.point_at_parameter(t);

        let edge0 = self.v1 - self.v0;
        let vp0 = plane_hit - self.v0;
        let c = edge0.cross(vp0);
        if self.normal.dot(c) < 0.0 {
            return None
        }

        let edge1 = self.v2 - self.v1;
        let vp1 = plane_hit - self.v1;
        let c1 = edge1.cross(vp1);
        if self.normal.dot(c1) < 0.0 {
            return None
        }

        let edge2 = self.v0 - self.v2;
        let vp2 = plane_hit - self.v2;
        let c2 = edge2.cross(vp2);
        if self.normal.dot(c2) < 0.0 {
            return None
        }

        Some((SurfaceInteraction::new(t, plane_hit, self.normal), &self.material))
    }
}