use std::io::Write;

use crate::math::Color;

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub id: u32,
    pub image: Image,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width: width,
            height: height,
            data: vec![Color::black(); width * height as usize],
        }
    }

    pub fn get_tile(&self, x: u32, y: u32, width: usize, height: usize) -> Tile {
        assert!(x as usize + width <= self.width);
        assert!(y as usize + height <= self.height);

        let mut tile = Tile {
            x: x,
            y: y,
            id: 0,
            image: Image::new(width, height),
        };
        for u in 0..width as u32 {
            for v in 0..height as u32 {
                tile.image.put_pixel(u, v, *self.get_pixel(x + u, y + v));
            }
        }
        tile
    }

    pub fn split_into_tiles(&self, tiles_w: usize, tiles_h: usize) -> Vec<Tile> {
        let size_w = self.width / tiles_w;
        let size_h = self.height / tiles_h;
        assert_eq!(size_w * tiles_w, self.width);
        assert_eq!(size_h * tiles_h, self.height);

        let mut tiles = vec![];
        for u in 0..tiles_w {
            for v in 0..tiles_h {
                let mut tile = self.get_tile((u * size_w) as u32, (v * size_h) as u32, size_w, size_h);
                tile.id = (u + v*tiles_w) as u32;
                tiles.push(tile);
            }
        }
        tiles
    }

    pub fn set_tile(&mut self, tile: &Tile) {
        assert!(tile.x + tile.image.width() <= self.width());
        assert!(tile.y + tile.image.height() <= self.height());

        for u in 0..tile.image.width() {
            for v in 0..tile.image.height() {
                self.put_pixel(tile.x + u, tile.y + v, *tile.image.get_pixel(u, v));
            }
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        assert!(x < self.width as u32);
        assert!(y < self.height as u32);
        self.data[self.width * y as usize + x as usize] = color;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &Color {
        &self.data[self.width * y as usize + x as usize]
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn height(&self) -> u32 {
        self.height as u32
    }

    pub fn gamma_correct(&mut self, gamma: f32) {
        for i in 0..self.width * self.height {
            self.data[i].r = self.data[i].r.powf(1.0 / gamma);
            self.data[i].g = self.data[i].g.powf(1.0 / gamma);
            self.data[i].b = self.data[i].b.powf(1.0 / gamma);
        }
    }

    pub fn write_ppm(&self, filename: String) {
        let mut file = std::fs::File::create(filename).expect("File creation failed.");
        file.write_all(format!("P3\n{} {}\n255\n  ", self.width, self.height).as_bytes())
            .expect("File writing failed.");

        for i in 0..self.width * self.height {
            file.write_all(self.write_pixel(i).as_bytes())
                .expect("File writing failed.");
        }
    }

    fn write_pixel(&self, idx: usize) -> String {
        let image_color = self.data[idx].to_u8();
        format!("{} {} {}\n", image_color.0, image_color.1, image_color.2)
    }
}

impl Tile {
    pub fn tile_to_image_coordinates(&self, x: u32, y: u32) -> (u32, u32) {
        (self.x + x, self.y + y)
    }
}
