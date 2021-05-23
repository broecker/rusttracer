
use std::ops;
use rand::Rng;

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

  pub fn from_vec3(v: &Vec3) -> Color {
    return Color {
      r: v.x,
      g: v.y,
      b: v.z
    }
  }

  pub fn to_u8(&self) -> (u8,u8,u8) {
    return ((self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8);
  }

  pub fn black() -> Color {
    return Color {r: 0.0, g:0.0, b:0.0 }
  }
}

impl Vec3 {
  pub fn mag(&self) -> f32 {
    self.mag_squared().sqrt()
  }

  pub fn mag_squared(&self) -> f32 {
    return Vec3::dot(self, self)
  }

  pub fn normalized(&self) -> Vec3 {
    *self / self.mag()
  }

  pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    a.x*b.x + a.y*b.y + a.z*b.z
  }

  pub fn zero() -> Vec3 {
    return Vec3{x: 0.0, y: 0.0, z: 0.0}
  }

  pub fn random() -> Vec3 {
    let mut rng = rand::thread_rng();
    return Vec3{x: rng.gen_range(-1.0..1.0), y: rng.gen_range(-1.0..1.0), z: rng.gen_range(-1.0..1.0)};
  }

  pub fn random_in_unit_sphere() -> Vec3 {
    loop {
      let v = Vec3::random();
      if v.mag_squared() < 1.0 {
        return v
      }
    }
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

impl ops::MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, _rhs: f32) {
    self.x *= _rhs;
    self.y *= _rhs;
    self.z *= _rhs;
  }
}

impl ops::Div<f32> for Vec3 {
  type Output = Vec3;
  fn div(self, _rhs: f32) -> Vec3 {
    Vec3{x: self.x/_rhs, y: self.y/_rhs, z: self.z/_rhs}
  }
}

impl ops::Add<Color> for Color {
  type Output = Color;
  fn add(self, _rhs: Color) -> Color {
    Color{r: self.r + _rhs.r, g: self.g + _rhs.g, b: self.b + _rhs.b}
  }
}

impl ops::AddAssign<Color> for Color {
  fn add_assign(&mut self, _rhs: Color) {
    self.r += _rhs.r;
    self.g += _rhs.g;
    self.b += _rhs.b;
  }
}

impl ops::Div<f32> for Color {
  type Output = Color;
  fn div(self, _rhs: f32) -> Color {
    Color{r: self.r/_rhs, g: self.g/_rhs, b: self.b/_rhs}
  }
}

impl ops::Mul<f32> for Color {
  type Output = Color;
  fn mul(self, _rhs: f32) -> Color {
    Color{r: self.r*_rhs, g: self.g*_rhs, b: self.b*_rhs}
  }
}

impl ops::MulAssign<f32> for Color {
  fn mul_assign(&mut self, _rhs: f32) {
    self.r *= _rhs;
    self.g *= _rhs;
    self.b *= _rhs;
  }
}