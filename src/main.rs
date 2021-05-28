
mod camera;
mod image;
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

fn ray_color(ray: &Ray, world: &IntersectableList<Sphere>, depth: i32) -> Color {
  let mut hit_record = HitRecord::new();

  if depth <= 0 {
    return Color{r: 0.0, g: 0.0, b: 0.0};
  }

  if world.intersect(ray, 0.0001, 10000.0, &mut hit_record) {
    let target = hit_record.point + hit_record.normal + Vec3::random_in_unit_sphere().normalized();
    return ray_color(&Ray{origin: hit_record.point, direction: target - hit_record.point}, world, depth-1) * 0.5;
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
    let mut image = image::Image::new(512, 512);
    let samples_per_pixel = 25;
    let max_depth = 50;
    let gamma = 2.0;

    // World
    let mut world = IntersectableList::<Sphere>::new();
    world.add(Sphere::new(Vec3{x:  0.5, y: 0.0, z: -1.0}, 0.5));
    world.add(Sphere::new(Vec3{x: -0.5, y: 0.0, z: -1.0}, 0.5));
    world.add(Sphere::new(Vec3{x: 0.0, y: -100.5, z: -1.0}, 100.0));

    // Camera
    let camera = Camera::new(image.aspect_ratio());

    // Render!
    for mut tile in image.split_into_tiles(4, 4) {
      println!("Starting tile {}{}", tile.x, tile.y);
      for j in 0..tile.image.height() {
        let progress = ((j as f32 / tile.image.height() as f32) * 100.0) as u32;
        println!("Progress: {}% (line {})", progress, j);

        for i in 0..tile.image.width() {
          let mut color = Color{r:0.0, g:0.0, b:0.0};
          for _sample in 0..samples_per_pixel {
            let u = (i as f32 + rng.gen_range(0.0..1.0)) / (tile.image.width() as f32 - 1.0);
            let v = 1.0 - (j as f32 + rng.gen_range(0.0..1.0)) / (tile.image.height() as f32 - 1.0);

            let ray = camera.get_ray(u, v);
            color += ray_color(&ray, &world, max_depth);
          }

          tile.image.put_pixel(i, j, color / samples_per_pixel as f32);
        }
      }
      image.set_tile(&tile);
    }

    // File output.
    image.gamma_correct(gamma);
    image.write_ppm("output.ppm".to_string());
}
