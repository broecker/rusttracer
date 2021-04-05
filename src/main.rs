
mod math;

use math::Color;
use math::Ray;
use math::Vec3;

fn ray_color(ray: &Ray) -> Color {
  let unit_direction = ray.direction.normalized();
  let t = 0.5 * (unit_direction.y + 1.0);
  let white = Color{r: 1.0, g: 1.0, b: 1.0 };
  let blueish = Color{r: 0.5, g: 0.7, b: 1.0};

  return Color::lerp(&white, &blueish, t)
}

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;
    let aspect_ratio = image_width as f32 / image_height as f32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3{x:0.0, y:0.0, z:0.0};
    let horizontal = Vec3{x:viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{x:0.0, y:viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{x:0.0, y:0.0, z:focal_length};

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
          let u = i as f32 / (image_width + 1) as f32;
          let v = 1.0 - j as f32 / (image_height + 1) as f32;

          let ray = Ray{ origin: origin, direction: lower_left_corner + horizontal*u + vertical*v - origin };
          let color = ray_color(&ray).to_u8();
          println!("{} {} {}", color.0, color.1, color.2);
        }
    }
}
