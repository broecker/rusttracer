
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

  pub fn from(r: f32, g: f32, b: f32) -> Color {
    return Color{r,g,b};
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
