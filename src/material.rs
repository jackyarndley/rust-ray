use crate::vec3::Vec3;
use rand::{thread_rng, Rng};
use crate::ray::Ray;

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * v.dot(n)
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, refraction: f64) -> f64 {
    let r0 = ((1.0 - refraction) / (1.0 + refraction)).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

// The attenuation here is the amount of each RGB colour which is dissipated with each bounce
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
    }
}

impl Material {
    pub fn scatter(&self, r: Ray, n: Vec3, p: Vec3) -> (Ray, Vec3, bool) {
        match self {
            Material::Lambertian {
                attenuation
            } => {
                let target = p + n + Vec3::random_in_unit_sphere(thread_rng());
                (Ray::new(p, target - p), *attenuation, true)
            }
            Material::Metal {
                attenuation,
                fuzziness
            } => {
                let reflected = reflect(r.direction.unit(), n);
                let scattered = Ray::new(p, reflected + Vec3::random_in_unit_sphere(thread_rng()) * *fuzziness);

                // Check to make sure the ray is not reflecting in the same direction
                let b = scattered.direction.dot(n) > 0.0;

                (scattered, *attenuation, b)
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
                let attenuation = Vec3::new(1.0, 1.0, 1.0);
                (scattered, attenuation, true)
            }
        }
    }
}