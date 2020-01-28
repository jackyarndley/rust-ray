use crate::util::{Ray, Vec3};
use crate::objects::Hitable;

pub struct Sample<'a> {
    ray: Option<Ray>,
    color: Vec3,
    depth: usize,
    max_depth: usize,
    world: &'a dyn Hitable
}

impl<'a> Sample<'a> {
    pub fn new(ray: Ray, max_depth: usize, world: &'a dyn Hitable) -> Self {
        Sample {
            ray: Some(ray),
            color: Vec3::new(1.0, 1.0, 1.0),
            depth: 0,
            max_depth,
            world
        }
    }
}

impl<'a> Iterator for Sample<'a> {
    type Item = Vec3;

    // Propagates the ray through the scene to get the color of the sample
    fn next(&mut self) -> Option<Vec3> {
        match self.ray {
            Some(ray) => {
                // t_min here is set to 0.001 to prevent some shadowing errors
                match self.world.hit(ray, 0.001, std::f64::INFINITY) {
                    Some((surface_interaction, material)) => {
                        match material.scatter(ray, surface_interaction.normal, surface_interaction.point) {
                            (attenuation, None) => {
                                self.color *= attenuation;
                                self.ray = None;
                            }
                            (attenuation, Some(scattered)) => {
                                self.color *= attenuation;

                                if self.depth < self.max_depth {
                                    self.depth += 1;
                                    self.ray = Some(scattered);
                                } else {
                                    self.ray = None
                                }
                            }
                        }
                    }
                    None => {
                        // This is the sky color and falloff
                        let unit_direction: Vec3 = ray.direction.unit();
                        let t: f64 = 0.5 * (unit_direction.y + 1.0);
                        self.color *= Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
                        self.ray = None;
                    }
                }
                Some(self.color)
            }
            None => {
                None
            }
        }
    }
}