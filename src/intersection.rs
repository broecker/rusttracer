
use crate::math::Ray;
use crate::math::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord {
  // Intersection point in world coordinates.
  pub point: Vec3,
  // Normal at intersection point in world coordinates.
  pub normal: Vec3,
  // Ray march param.
  pub t: f32,
  // Set to true if the ray hit the front facing.
  pub front_face: bool,
}

// All the objects we can intersect.
pub trait Intersectable {
  fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut HitRecord) -> bool;
}

pub struct Sphere {
  center: Vec3,
  radius: f32,
}

// See: https://bennetthardwick.com/blog/dont-use-boxed-trait-objects-for-struct-internals/
pub struct IntersectableList<I: Intersectable> {
  objects: Vec<I>,
}


impl Sphere {
  pub fn new(center: Vec3, radius: f32) -> Sphere {
    return Sphere{center: center, radius: radius}
  }
}

impl HitRecord {
  pub fn new() -> HitRecord {
    return HitRecord{
      point: Vec3::zero(),
      normal: Vec3::zero(),
      t: 0.0,
      front_face: false,
    }
  }

  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
    self.front_face = Vec3::dot(&ray.direction, &outward_normal) < 0.0;
    self.normal = outward_normal;
    if !self.front_face {
      self.normal *= -1.0
    }
  }
}

impl Intersectable for Sphere {
  fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut HitRecord) -> bool {
    let oc = ray.origin - self.center;
    let a = ray.direction.mag_squared();
    let half_b = Vec3::dot(&oc, &ray.direction);
    let c = oc.mag_squared() - self.radius*self.radius;

    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
      return false
    }
    let sqrt_d = discriminant.sqrt();

    // Find the nearest root that lies in the acceptable range.
    let mut root = (-half_b - sqrt_d) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrt_d) / a;
      if root < t_min || t_max < root {
        return false
      }
    }

    hit.t = root;
    hit.point = ray.at(root);
    let normal = (hit.point - self.center) / self.radius;
    hit.set_face_normal(ray, normal);
    return true
  }
}

impl<I: Intersectable> IntersectableList<I> {
  pub fn new() -> IntersectableList<I> {
    return IntersectableList{objects: Vec::new()};
  }

  pub fn add(&mut self, object: I) {
    self.objects.push(object);
  }
}

impl<I:Intersectable> Intersectable for IntersectableList<I> {
  fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut HitRecord) -> bool {
    let mut any_hit = false;
    let mut closest = t_max;
    let mut record: HitRecord = HitRecord::new();
    for obj in self.objects.iter() {
      if obj.intersect(&ray, t_min, closest, &mut record) {
        any_hit = true;
        closest = record.t;
        *hit = record;
      }
    }
    any_hit
  }
}
