
use crate::math::Ray;
use crate::math::Vec3;

pub fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {
  let oc = ray.origin - *center;
  let a = Vec3::dot(&ray.direction, &ray.direction);
  let b = 2.0 * Vec3::dot(&oc, &ray.direction);
  let c = Vec3::dot(&oc, &oc) - radius*radius;
  let discriminant = b*b - 4.0*a*c;
  if discriminant < 0.0 {
    return -1.0
  } else {
    return (-b - discriminant.sqrt()) / (2.0 * a)
  }
}


