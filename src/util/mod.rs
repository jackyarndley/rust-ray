use crate::hitable::{HitableList, Hitable, Sphere};
use crate::material::Material;
use crate::vec3::Vec3;
use rand::{Rng, thread_rng};

// Clamps a value between two bounds
pub fn clamp<T: PartialOrd>(value: T, lower: T, upper: T) -> T {
    if value < lower {
        lower
    } else if value > upper {
        upper
    } else {
        value
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();
    let mut p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);

    while p.squared_length() >= 1.0 {
        p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
    }
    p
}

pub fn random_scene() -> HitableList {
    let mut rng = thread_rng();
    let mut list: Vec<Box<dyn Hitable>> = vec![];
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian {attenuation: Vec3::new(0.5, 0.5, 0.5)})));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.75 {
                    list.push(Box::new(Sphere::new(center, 0.2, Material::Lambertian {attenuation: Vec3::new(rng.gen::<f64>() * rng.gen::<f64>(), rng.gen::<f64>() * rng.gen::<f64>(), rng.gen::<f64>() * rng.gen::<f64>())})));
                } else if choose_mat < 0.90 {
                    list.push(Box::new(Sphere::new(center, 0.2, Material::Metal {attenuation: Vec3::new(0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>())), fuzziness: 0.5 * rng.gen::<f64>()})));
                } else if choose_mat < 0.975 {
                    list.push(Box::new(Sphere::new(center, 0.2, Material::Dielectric {refraction: 1.5})));
                } else {
                    list.push(Box::new(Sphere::new(center, 0.2, Material::Emission {color: Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 10.0})));
                }
            }
        }
    }

    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::Dielectric {refraction: 1.5})));
    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Material::Lambertian {attenuation: Vec3::new(0.4, 0.2, 0.1)})));
    list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Material::Metal {attenuation: Vec3::new(0.7, 0.6, 0.5), fuzziness: 0.0})));

    HitableList::new(list)
}