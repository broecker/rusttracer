

use crate::intersection::HitRecord;
use crate::math::Color;
use crate::math::Ray;
use crate::math::Vec3;

pub trait Material: MaterialClone + Send {
  fn scatter(&self, ray_in: &Ray, hit: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

// See: https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
pub trait MaterialClone {
  fn clone_box(&self) -> Box<dyn Material>;
}

#[derive(Clone, Copy)]
pub struct Constant {
  pub color: Color,
}

#[derive(Clone, Copy)]
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

impl<T> MaterialClone for T where T: 'static + Material + Clone {
  fn clone_box(&self) -> Box<dyn Material> {
    Box::new(self.clone())
  }
}

impl Clone for Box<dyn Material> {
  fn clone(&self) -> Box<dyn Material> {
    self.clone_box()
  }
}

impl Material for Constant {
  fn scatter(&self, _ray_in: &Ray, _hit: &HitRecord, attenuation: &mut Color, _scattered: &mut Ray) -> bool {
    *attenuation = self.color;
    false
  }
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
