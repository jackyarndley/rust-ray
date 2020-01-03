use crate::vec3::Vec3;
use rand::thread_rng;
use crate::ray::Ray;

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * v.dot(n)
}

// The attenuation here is the amount of each RGB colour which is dissipated with each bounce
pub enum Material {
    Lambertian {
        attenuation: Vec3
    },
    Metal {
        attenuation: Vec3,
        fuzziness: f64
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
        }
    }
}