
use std::io::Write;

mod camera;
mod intersection;
mod math;

use math::Color;
use math::Ray;
use math::Vec3;

use rand::Rng;

use intersection::HitRecord;
use intersection::Sphere;
use crate::intersection::Intersectable;
use crate::intersection::IntersectableList;
use camera::Camera;

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
    // RNG
    let mut rng = rand::thread_rng();

    // Image
    let image_width = 512;
    let image_height = 512;
    let aspect_ratio = image_width as f32 / image_height as f32;
    let samples_per_pixel = 200;

    // World
    let mut world = IntersectableList::<Sphere>::new();
    world.add(Sphere::new(Vec3{x: 0.0, y: 0.0, z: -1.0}, 0.5));
    world.add(Sphere::new(Vec3{x: 0.0, y: -100.5, z: -1.0}, 100.0));

    // Camera
    let camera = Camera::new(aspect_ratio);

    // File output.
    let mut file = std::fs::File::create("output.ppm").expect("File creation failed.");
    file.write_all(format!("P3\n{} {}\n255\n  ", image_width, image_height).as_bytes()).expect("File writing failed.");

    // Render!
    for j in 0..image_height {
      let progress = ((j as f32 / image_height as f32) * 100.0) as u32;
      println!("Progress: {}% (line {})", progress, j);

      for i in 0..image_width {
        let mut color = Color{r:0.0, g:0.0, b:0.0};
        for _sample in 0..samples_per_pixel {
          let u = (i as f32 + rng.gen_range(0.0..1.0)) / (image_width as f32 - 1.0);
          let v = 1.0 - (j as f32 + rng.gen_range(0.0..1.0)) / (image_height as f32 - 1.0);

          let ray = camera.get_ray(u, v);
          let ray_color = ray_color(&ray, &world);
          color.r += ray_color.r;
          color.g += ray_color.g;
          color.b += ray_color.b;
        }

        color.r = color.r / samples_per_pixel as f32;
        color.g = color.g / samples_per_pixel as f32;
        color.b = color.b / samples_per_pixel as f32;

        let image_color = color.to_u8();
        file.write_all(format!("{} {} {}\n", image_color.0, image_color.1, image_color.2).as_bytes()).expect("File writing failed.");
      }
    }
}
