use crate::objects::{HitableList, Sphere, Hitable, Triangle};
use crate::material::Material;
use crate::util::{load_model, Vec3, Camera};

use rand::{thread_rng, Rng};

pub fn simple_scene(width: usize, height: usize) -> (Camera, HitableList) {
    let look_from = Vec3::new(16.0, 4.0, 0.0);
    let look_at = Vec3::new(0.0, 0.8, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.0;

    let camera = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 12.0, width as f64 / height as f64, aperture, dist_to_focus);

    let mut list: Vec<Box<dyn Hitable>> = vec![];
    list.push(Box::new(Triangle::new2(Vec3::new(1000.0, 0.0, 1000.0), Vec3::new(-1000.0, 0.0, 1000.0), Vec3::new(1000.0, 0.0, -1000.0), Vec3::new(0.0, 1.0, 0.0), Material::Lambertian {attenuation: Vec3::new(0.5, 0.5, 0.5)})));
    list.push(Box::new(Triangle::new2(Vec3::new(-1000.0, 0.0, -1000.0), Vec3::new(1000.0, 0.0, -1000.0), Vec3::new(-1000.0, 0.0, 1000.0), Vec3::new(0.0, 1.0, 0.0), Material::Lambertian {attenuation: Vec3::new(0.5, 0.5, 0.5)})));

    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, -3.375), 1.0, Material::Dielectric {refraction: 1.5})));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, -1.125), 1.0, Material::Metal {attenuation: Vec3::new(212.0 / 255.0, 105.0 / 255.0, 33.0 / 255.0), fuzziness: 0.0})));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 1.125), 1.0, Material::Lambertian {attenuation: Vec3::new(167.0 / 255.0, 51.0 / 255.0, 0.0)})));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 3.375), 1.0, Material::Emission {color: Vec3::new(227.0 / 255.0 * 4.0, 160.0 / 255.0 * 4.0, 1.0)})));

    (camera, HitableList::new(list))
}

pub fn random_scene() -> (Camera, HitableList) {
    let look_from = Vec3::new(16.0, 2.0, 4.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.2;

    let width: usize = 1200;
    let height: usize = 500;

    let camera = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 15.0, width as f64 / height as f64, aperture, dist_to_focus);

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

    (camera, HitableList::new(list))
}

pub fn random_scene2() -> (Camera, HitableList) {
    let look_from = Vec3::new(16.0, 2.0, 4.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.2;

    let width: usize = 1200;
    let height: usize = 500;

    let camera = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 15.0, width as f64 / height as f64, aperture, dist_to_focus);

    let mut list: Vec<Box<dyn Hitable>> = vec![];

    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::Dielectric {refraction: 1.5})));
    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Material::Metal {attenuation: Vec3::new(0.4, 0.2, 0.1), fuzziness: 0.0})));

    load_model(&mut list, "untitled.obj");

    (camera, HitableList::new(list))
}