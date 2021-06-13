use crate::math::Color;
use crate::math::Vec3;

use rand::Rng;

use std::sync::Arc;

use crate::intersection::IntersectableList;
use crate::intersection::Sphere;

use crate::material;

pub fn make_random_sphere_scene() -> IntersectableList<Sphere> {
    let mut rng = rand::thread_rng();
    let mut world = IntersectableList::new();

    let material_glass = Arc::new(material::Dielectric {
        index_of_refraction: 1.5,
    });

    for x in -11..11 {
        for z in -11..11 {
            let center = Vec3 {
                x: x as f32 + 0.9 * rng.gen_range(0.0..1.0),
                y: 0.2,
                z: z as f32 + 0.9 * rng.gen_range(0.0..1.0),
            };

              if (center - Vec3{x:4.0, y: 0.2, z:0.0}).mag() > 0.9 {

              let choose_mat = rng.gen_range(0.0..1.0);
              if choose_mat < 0.8 {
                  // Diffuse with a random color.
                  world.add(Sphere::new(
                      center,
                      0.2,
                      Arc::new(material::Lambertian {
                          albedo: Color::random(),
                      }),
                  ));
              } else if choose_mat < 0.95 {
                  // Metal with random color and roughness.
                  world.add(Sphere::new(
                      center,
                      0.2,
                      Arc::new(material::Metal {
                          albedo: Color::random(),
                          roughness: rng.gen_range(0.01..0.2),
                      }),
                  ));
              } else {
                  // Glass
                  world.add(Sphere::new(center, 0.2, material_glass.clone()));
              }
            }
        }
    }
    world
}
