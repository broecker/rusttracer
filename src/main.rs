
mod camera;
mod image;
mod intersection;
mod material;
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

extern crate scoped_threadpool;
use scoped_threadpool::Pool;

struct RenderSettings {
  samples_per_pixel: u32, 
  max_recursion_depth: u32,
  image_gamma: f32,
  render_threads: u32
}


fn ray_color(ray: &Ray, world: &IntersectableList<Sphere>, depth: u32) -> Color {
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

fn trace(image: &mut image::Image, camera: &Camera, world: &IntersectableList<Sphere>, render_settings: &RenderSettings) { 
  let image_w = image.width();
  let image_h = image.height();

  // Threading
  let mut pool = Pool::new(render_settings.render_threads);

  // Render!    
  let mut tiles = image.split_into_tiles(16, 16);
  pool.scoped(|scope| {
    for tile in &mut tiles {
      scope.execute(move || {
        // RNG
        let mut rng = rand::thread_rng();

        for j in 0..tile.image.height() {
          print!(".");
          for i in 0..tile.image.width() {
            let mut color = Color{r:0.0, g:0.0, b:0.0};
            for _sample in 0..render_settings.samples_per_pixel {
              let coord = tile.tile_to_image_coordinates(i, j);

              let u = (coord.0 as f32 + rng.gen_range(0.0..1.0)) / (image_w as f32 - 1.0);
              let v = 1.0 - (coord.1 as f32 + rng.gen_range(0.0..1.0)) / (image_h as f32 - 1.0);

              let ray = camera.get_ray(u, v);
              color += ray_color(&ray, &world, render_settings.max_recursion_depth);
            }

            tile.image.put_pixel(i, j, color / render_settings.samples_per_pixel as f32);
          }
        }

      });
    }
  });

  for tile in &tiles {
    image.set_tile(tile);
  }
}


fn main() {
    // Image
    let mut image = image::Image::new(512, 512);

    let render_settings = RenderSettings{samples_per_pixel: 500, max_recursion_depth: 50, image_gamma: 2.0, render_threads: 16};

    // World
    let mut world = IntersectableList::<Sphere>::new();
    world.add(Sphere::new(Vec3{x:  0.5, y: 0.0, z: -1.0}, 0.5));
    world.add(Sphere::new(Vec3{x: -0.5, y: 0.0, z: -1.0}, 0.5));
    world.add(Sphere::new(Vec3{x: 0.0, y: -100.5, z: -1.0}, 100.0));

    // Camera
    let camera = Camera::new(image.aspect_ratio());

    trace(&mut image, &camera, &world, &render_settings);


    // File output.
    image.gamma_correct(render_settings.image_gamma);
    image.write_ppm("output.ppm".to_string());
}
