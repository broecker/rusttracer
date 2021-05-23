use std::io::Write;

use crate::math::Color;

pub struct Image {
  width: usize,
  height: usize, 
  data: Vec<Color>
}

impl Image {
  pub fn new(width: usize, height: usize) -> Image {    
    return Image {
      width: width, height: height, data: vec![Color::black(); width*height as usize]
    }
  }

  pub fn aspect_ratio(&self) -> f32 {
    return self.width as f32 / self.height as f32
  }

  pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
    assert!(x < self.width as u32);
    assert!(y < self.height as u32);
    self.data[self.width*y as usize + x as usize] = color;
  }

  pub fn width(&self) -> u32 {
    return self.width as u32
  }

  pub fn height(&self) -> u32 {
    return self.height as u32
  }

  pub fn gamma_correct(&mut self, gamma: f32) {
    for i in 0..self.width*self.height {
      self.data[i].r = self.data[i].r.powf(1.0 / gamma);
      self.data[i].g = self.data[i].g.powf(1.0 / gamma);
      self.data[i].b = self.data[i].b.powf(1.0 / gamma);
    }
  }

  pub fn write_ppm(&self, filename: String) {
    let mut file = std::fs::File::create(filename).expect("File creation failed.");
    file.write_all(format!("P3\n{} {}\n255\n  ", self.width, self.height).as_bytes()).expect("File writing failed.");

    for i in 0..self.width*self.height {
      file.write_all(self.write_pixel(i).as_bytes()).expect("File writing failed.");
    }
  }

  fn write_pixel(&self, idx: usize) -> String {
    let image_color = self.data[idx].to_u8();
    return format!("{} {} {}\n", image_color.0, image_color.1, image_color.2);
  }
}