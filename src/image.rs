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

  // Create a sub image from x,y with width,height dimensions.
  pub fn sub_image(&self, x: u32, y: u32, width: usize, height: usize) -> Image {
    assert!(x as usize + width < self.width);
    assert!(y as usize + height < self.height);

    let mut tile = Image::new(width, height);
    for u in 0..width as u32 {
      for v in 0..height as u32 {
        tile.put_pixel(u, v, *self.get_pixel(x + u, y + v) );
      }
    }
    tile
  }

  pub fn split(&self, tiles_w: usize, tiles_h: usize) -> Vec<Image> {
    let size_w = self.width / tiles_w;
    let size_h = self.height / tiles_h;
    assert_eq!(size_w * tiles_w, self.width);
    assert_eq!(size_h * tiles_h, self.height);

    let mut tiles = vec!();

    for u in 0..tiles_w {
      for v in 0..tiles_h {
        tiles.push(self.sub_image((u*size_w) as u32, (v*size_h) as u32, size_w, size_h));
      }
    }

    tiles
  }

  pub fn set_tile(&mut self, x: u32, y: u32, tile: &Image) {
    assert!(x + tile.width() < self.width());
    assert!(y + tile.height() < self.height());

    for u in 0..tile.width() {
      for v in 0..tile.height() {
        self.put_pixel(x + u, y + v, *tile.get_pixel(u, v));
      }
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

  pub fn get_pixel(&self, x: u32, y: u32) -> &Color {
    &self.data[self.width*y as usize + x as usize]
  }

  pub fn width(&self) -> u32 {
    self.width as u32
  }

  pub fn height(&self) -> u32 {
    self.height as u32
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
    format!("{} {} {}\n", image_color.0, image_color.1, image_color.2)
  }
}