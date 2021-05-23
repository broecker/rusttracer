
use std::io::Write;

mod intersection;
mod math;

use math::Color;
use math::Ray;
use math::Vec3;

use intersection::HitRecord;
use intersection::Sphere;
use crate::intersection::Intersectable;
use crate::intersection::IntersectableList;

fn ray_color(ray: &Ray, world: &IntersectableList<Sphere>) -> Color {
  let mut hit_record = HitRecord::new();

  if world.intersect(ray, 0.0, 10000.0, &mut hit_record) {
    let mut n = hit_record.normal.clone();
    n += Vec3::one();
    n *= 0.5;
    return Color::from_vec3(&n)
  }

  let unit_direction = ray.direction.normalized();
  let t = 0.5 * (unit_direction.y + 1.0);
  let white = Color{r: 1.0, g: 1.0, b: 1.0 };
  let blueish = Color{r: 0.5, g: 0.7, b: 1.0};

  return Color::lerp(&white, &blueish, t)
}

fn main() {
    // Image
    let image_width = 512;
    let image_height = 512;
    let aspect_ratio = image_width as f32 / image_height as f32;

    // World
    let mut world = IntersectableList::<Sphere>::new();
    world.add(Sphere::new(Vec3{x: 0.0, y: 0.0, z: -1.0}, 0.5));
    world.add(Sphere::new(Vec3{x: 0.0, y: -100.5, z: -1.0}, 100.0));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3{x:0.0, y:0.0, z:0.0};
    let horizontal = Vec3{x:viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{x:0.0, y:viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{x:0.0, y:0.0, z:focal_length};

    // File output.
    let mut file = std::fs::File::create("output.ppm").expect("File creation failed.");
    file.write_all(format!("P3\n{} {}\n255\n  ", image_width, image_height).as_bytes()).expect("File writing failed.");

    // Render!
    for j in 0..image_height {
      let progress = ((j as f32 / image_height as f32) * 100.0) as u32;
      println!("Progress: {}% (line {})", progress, j);

      for i in 0..image_width {
        let u = i as f32 / (image_width + 1) as f32;
        let v = 1.0 - j as f32 / (image_height + 1) as f32;

        let ray = Ray{ origin: origin, direction: lower_left_corner + horizontal*u + vertical*v - origin };
        let color = ray_color(&ray, &world).to_u8();
        file.write_all(format!("{} {} {}\n", color.0, color.1, color.2).as_bytes()).expect("File writing failed.");
      }
    }
}
