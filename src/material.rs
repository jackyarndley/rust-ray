use crate::vec3::Vec3;
use rand::{thread_rng, Rng};
use crate::ray::Ray;
use crate::util::{reflect, refract, schlick};

// The attenuation here is the amount of each RGB colour which is dissipated with each bounce
#[derive(Copy, Clone)]
pub enum Material {
    Lambertian {
        attenuation: Vec3
    },
    Metal {
        attenuation: Vec3,
        fuzziness: f64
    },
    Dielectric {
        refraction: f64
    },
    Emission {
        color: Vec3
    }
}

impl Material {
    pub fn scatter(&self, r: Ray, n: Vec3, p: Vec3) -> (Vec3, Option<Ray>) {
        match self {
            Material::Lambertian {
                attenuation
            } => {
                let target = p + n + Vec3::random_in_unit_sphere(thread_rng());
                (*attenuation, Some(Ray::new(p, target - p)))
            }
            Material::Metal {
                attenuation,
                fuzziness
            } => {
                let reflected = reflect(r.direction.unit(), n);
                let scattered = Ray::new(p, reflected + Vec3::random_in_unit_sphere(thread_rng()) * *fuzziness);

                // Check to make sure the ray is not reflecting in the same direction
                if scattered.direction.dot(n) > 0.0 {
                    (*attenuation, Some(scattered))
                } else {
                    (Vec3::new(0.0, 0.0, 0.0), None)
                }
            }
            Material::Dielectric {
                refraction
            } => {
                let reflected = reflect(r.direction, n);

                let (outward_normal, ni_over_nt, cosine) = if r.direction.dot(n) > 0.0 {
                    (
                        -n,
                        *refraction,
                        *refraction * r.direction.dot(n) / r.direction.length()
                    )
                } else {
                    (
                        n,
                        1.0 / *refraction,
                        -(r.direction.dot(n)) / r.direction.length()
                    )
                };

                let scattered = match refract(r.direction, outward_normal, ni_over_nt) {
                    Some(refracted) => {
                        let reflect_prob = schlick(cosine, *refraction);
                        let mut rng = thread_rng();
                        if rng.gen::<f64>() < reflect_prob {
                            Ray::new(p, reflected)
                        } else {
                            Ray::new(p, refracted)
                        }
                    }
                    None => Ray::new(p, reflected)
                };

                (Vec3::new(1.0, 1.0, 1.0), Some(scattered))
            }
            Material::Emission {
                color
            } => {
                (*color, None)
            }
        }
    }
}