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

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * v.dot(n)
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f64, refraction: f64) -> f64 {
    let r0 = ((1.0 - refraction) / (1.0 + refraction)).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub fn load_model(list: &mut Vec<Box<dyn Hitable>>, model_name: &str) {
    let mut rng = thread_rng();
    let path = Path::new(model_name);

    let obj = tobj::load_obj(path);
    let (models, materials) = obj.unwrap();

    for model in models {
        let mesh = model.mesh;

        let material = if model.name.starts_with("Cube") {
            let choose_mat = rng.gen::<f64>();
            if choose_mat < 0.75 {
                Material::Lambertian {
                    attenuation: Vec3::new(rng.gen::<f64>() * rng.gen::<f64>(), rng.gen::<f64>() * rng.gen::<f64>(), rng.gen::<f64>() * rng.gen::<f64>())
                }
            } else if choose_mat < 0.90 {
                Material::Metal {
                    attenuation: Vec3::new(0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>())),
                    fuzziness: 0.5 * rng.gen::<f64>()
                }
            } else if choose_mat < 0.975 {
                Material::Dielectric {
                    refraction: 1.5
                }
            } else {
                Material::Emission {
                    color: Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 10.0
                }
            }
        } else {
            match mesh.material_id {
                Some(id) => {
                    let material_info = &materials[id];
                    Material::Lambertian {
                        attenuation: Vec3::new(material_info.diffuse[0] as f64, material_info.diffuse[1] as f64, material_info.diffuse[2] as f64)
                    }
                },
                None => {
                    Material::Lambertian {
                        attenuation: Vec3::new(0.5, 0.5, 0.5)
                    }
                }
            }
        };

        for f in 0..mesh.indices.len() / 3 {
            let position_index = 3 * mesh.indices[3 * f] as usize;
            let position_index1 = 3 * mesh.indices[3 * f + 1] as usize;
            let position_index2 = 3 * mesh.indices[3 * f + 2] as usize;

            let element = Triangle::new2(
                Vec3::new(mesh.positions[position_index] as f64, mesh.positions[position_index + 1] as f64, mesh.positions[position_index + 2] as f64),
                Vec3::new(mesh.positions[position_index1] as f64, mesh.positions[position_index1 + 1] as f64, mesh.positions[position_index1 + 2] as f64),
                Vec3::new(mesh.positions[position_index2] as f64, mesh.positions[position_index2 + 1] as f64, mesh.positions[position_index2 + 2] as f64),
                Vec3::new(mesh.normals[position_index] as f64, mesh.normals[position_index + 1] as f64, mesh.normals[position_index + 2] as f64),
                material
            );

            list.push(Box::new(element));
        }
    }
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

    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::Dielectric {refraction: 1.5})));
    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Material::Metal {attenuation: Vec3::new(0.4, 0.2, 0.1), fuzziness: 0.0})));

    load_model(&mut list, "untitled.obj");

    HitableList::new(list)
}
