use crate::math::Vec3;

// An axis-aligned bounding box.

#[derive(Debug)]
pub struct AABB {
  pub min: Vec3,
  pub max: Vec3,

  initialized: bool,
}

fn component_wise_min(a: &Vec3, b: &Vec3) -> Vec3 {
  Vec3 {
    x: a.x.min(b.x),
    y: a.y.min(b.y),
    z: a.z.min(b.z),
  }
}

fn component_wise_max(a: &Vec3, b: &Vec3) -> Vec3 {
  Vec3 {
    x: a.x.max(b.x),
    y: a.y.max(b.y),
    z: a.z.max(b.z),
  }
}

impl AABB {
  pub fn new() -> AABB {
    AABB{min: Vec3::zero(), max: Vec3::zero(), initialized: false}
  }

  pub fn add_point(&mut self, pt: &Vec3) {
    if !self.initialized {
      self.min = pt.clone();
      self.max = pt.clone();
      self.initialized = true;
      return
    }

    self.min = component_wise_min(&self.min, &pt);
    self.max = component_wise_max(&self.max, &pt);
  }

  pub fn add_bbox(&mut self, other: &AABB) {
    if !self.initialized {
      self.min = other.min;
      self.max = other.max;
      self.initialized = true;
      return
    }

    self.min = component_wise_min(&self.min, &other.min);
    self.max = component_wise_max(&self.max, &other.max);
    self.initialized = true
  }

  pub fn add_sphere(&mut self, center: &Vec3, radius: f32) {
    let r = Vec3{x: radius, y: radius, z: radius};
    self.min = component_wise_min(&self.min, &(*center - r));
    self.max = component_wise_max(&self.max, &(*center + r));
  }

  pub fn extend(&self) -> Vec3 {
    Vec3 {
      x: self.max.x - self.min.x,
      y: self.max.y - self.min.y,
      z: self.max.z - self.min.z,
    }
  }

  pub fn center(&self) -> Vec3 {
    self.min + self.extend() / 2.0
  }
}

