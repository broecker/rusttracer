
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
  pub x : f32,
  pub y : f32,
  pub z : f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
  pub origin: Vec3,
  pub direction : Vec3
}

impl Color {
  pub fn lerp(a: &Color, b: &Color, t: f32) -> Color {
    let s = 1.0 - t;
    Color{
      r: s*a.r + t*b.r,
      g: s*a.g + t*b.g,
      b: s*a.b + t*b.b,
    }
  }

  pub fn to_u8(&self) -> (u8,u8,u8) {
    return ((self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8);
  }
}

impl Vec3 {
  pub fn mag_squared(&self) -> f32 {
    self.x*self.x + self.y*self.y + self.z*self.z
  }

  pub fn mag(&self) -> f32 {
    self.mag_squared().sqrt()
  }

  pub fn normalized(&self) -> Vec3 {
    *self / self.mag()
  }
}

impl Ray {
  pub fn at(&self, t: f32) -> Vec3 {
    self.origin + self.direction * t
  }
}

impl ops::Add<Vec3> for Vec3 {
  type Output = Vec3;
  fn add(self, _rhs: Vec3) -> Vec3 {
    Vec3{x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z}
  }
}

impl ops::AddAssign<Vec3> for Vec3 {
  fn add_assign(&mut self, _rhs: Vec3) {
    self.x += _rhs.x;
    self.y += _rhs.y;
    self.z += _rhs.z;
  }
}

impl ops::Sub<Vec3> for Vec3 {
  type Output = Vec3;
  fn sub(self, _rhs: Vec3) -> Vec3 {
    Vec3{x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z}
  }
}

impl ops::SubAssign<Vec3> for Vec3 {
  fn sub_assign(&mut self, _rhs: Vec3) {
    self.x -= _rhs.x;
    self.y -= _rhs.y;
    self.z -= _rhs.z;
  }
}

// Component-wise multiplication.
impl ops::Mul<Vec3> for Vec3 {
  type Output = Vec3;
  fn mul(self, _rhs: Vec3) -> Vec3 {
    Vec3{x: self.x*_rhs.x, y:self.y*_rhs.y, z:self.z*_rhs.z}
  }
}

impl ops::Mul<f32> for Vec3 {
  type Output = Vec3;
  fn mul(self, _rhs: f32) -> Vec3 {
    Vec3{x: self.x*_rhs, y: self.y*_rhs, z: self.z*_rhs}
  }
}

impl ops::Div<f32> for Vec3 {
  type Output = Vec3;
  fn div(self, _rhs: f32) -> Vec3 {
    Vec3{x: self.x/_rhs, y: self.y/_rhs, z: self.z/_rhs}
  }
}
