

use crate::intersection::HitRecord;
use crate::math::Color;
use crate::math::Ray;
use crate::math::Vec3;

pub trait Material {
  fn scatter(&self, ray_in: &Ray, hit: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
  pub albedo: Color,
}

fn get_scatter_direction(normal: Vec3) -> Vec3 {
  let mut scatter_direction = normal + Vec3::random_unit_vector();
  if scatter_direction.near_zero() {
    scatter_direction = normal;
  }
  scatter_direction
}

impl Material for Lambertian {
  fn scatter(&self, _ray_in: &Ray, hit: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
    let scatter_direction = get_scatter_direction(hit.normal);
    scattered.origin = hit.point;
    scattered.direction = scatter_direction;
    *attenuation = self.albedo;
    true
  }
}
