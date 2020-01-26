use crate::objects::{HitableList, Hitable, Sphere, Triangle};
use crate::material::Material;
use crate::vec3::Vec3;
use rand::{Rng, thread_rng};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

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

pub fn random_scene2() -> HitableList {
    let mut list: Vec<Box<dyn Hitable>> = vec![];
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian {attenuation: Vec3::new(0.5, 0.5, 0.5)})));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::Lambertian {attenuation: Vec3::new(0.1, 0.1, 1.0)})));
    list.push(Box::new(Sphere::new(Vec3::new(5.0, 0.0, 0.0), 1.0, Material::Lambertian {attenuation: Vec3::new(0.1, 1.0, 0.1)})));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0, Material::Lambertian {attenuation: Vec3::new(1.0, 0.1, 0.1)})));

    list.push(Box::new(Triangle::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 5.0), Vec3::new(5.0, 0.0, 0.0), Material::Lambertian {attenuation: Vec3::new(0.4, 0.2, 0.1)})));
    list.push(Box::new(Triangle::new2(Vec3::new(0.0, 1.0, -1.0), Vec3::new(0.86602497100830078, 1.0, 0.5), Vec3::new(-0.86602497100830078, 1.0, 0.5), Vec3::new(0.0, 1.0, 0.0), Material::Lambertian {attenuation: Vec3::new(0.4, 0.2, 0.1)})));

    let path = Path::new("untitled.obj");

    let teapot = tobj::load_obj(path);
    let (models, materials) = teapot.unwrap();

    let mesh = &models[0].mesh;

    for f in 0..mesh.indices.len() / 3 {
        let position_index = 3 * mesh.indices[3 * f] as usize;
        let position_index1 = 3 * mesh.indices[3 * f + 1] as usize;
        let position_index2 = 3 * mesh.indices[3 * f + 2] as usize;

        let element = Triangle::new2(
            Vec3::new(mesh.positions[position_index] as f64, mesh.positions[position_index + 1] as f64, mesh.positions[position_index + 2] as f64),
            Vec3::new(mesh.positions[position_index1] as f64, mesh.positions[position_index1 + 1] as f64, mesh.positions[position_index1 + 2] as f64),
            Vec3::new(mesh.positions[position_index2] as f64, mesh.positions[position_index2 + 1] as f64, mesh.positions[position_index2 + 2] as f64),
            Vec3::new(mesh.normals[position_index] as f64, mesh.normals[position_index + 1] as f64, mesh.normals[position_index + 2] as f64),
            Material::Lambertian {
                attenuation: Vec3::new(0.4, 0.4, 0.4),
            }
        );

//        list.push(Box::new(element));
    }

    HitableList::new(list)
}
