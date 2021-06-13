use crate::math::Color;
use crate::math::Vec3;

use rand::Rng;

use std::sync::Arc;

use crate::intersection::Intersectables;
use crate::intersection::Sphere;
use crate::intersection::Triangle;

use crate::material;

pub fn make_random_sphere_scene() -> Intersectables {
    let mut rng = rand::thread_rng();
    let mut world = Intersectables::new();

    let material_glass = Arc::new(material::Dielectric {
        index_of_refraction: 1.5,
    });

    for x in -21..21 {
        for z in -21..21 {
            let center = Vec3 {
                x: x as f32 + 0.9 * rng.gen_range(0.0..1.0),
                y: 0.2,
                z: z as f32 + 0.9 * rng.gen_range(0.0..1.0),
            };

              if (center - Vec3{x:4.0, y: 0.2, z:0.0}).mag() > 0.9 {

              let choose_mat = rng.gen_range(0.0..1.0);
              if choose_mat < 0.8 {
                  // Diffuse with a random color.
                  world.add(Box::new(Sphere::new(
                      center,
                      0.2,
                      Arc::new(material::Lambertian {
                          albedo: Color::random(),
                      }),
                  )));
              } else if choose_mat < 0.95 {
                  // Metal with random color and roughness.
                  world.add(Box::new(Sphere::new(
                      center,
                      0.2,
                      Arc::new(material::Metal {
                          albedo: Color::random(),
                          roughness: rng.gen_range(0.01..0.2),
                      }),
                  )));
              } else {
                  // Glass
                  world.add(Box::new(Sphere::new(center, 0.2, material_glass.clone())));
              }
            }
        }
    }
    world
}

pub fn make_ground() -> Intersectables {
    let mut triangles = Intersectables::new();

    let triangle_mat = Arc::new(material::Lambertian {
        albedo: Color::gray(0.4),
    });

    let a = Vec3 {
        x: -50.0,
        y: 0.0,
        z: -50.0,
    };
    let b = Vec3 {
        x: -50.0,
        y: 0.0,
        z: 50.0,
    };
    let c = Vec3 {
        x: 50.0,
        y: 0.0,
        z: 50.0,
    };
    let d = Vec3 {
        x: 50.0,
        y: 0.0,
        z: -50.0,
    };

    triangles.add(Box::new(Triangle {
        a: a, 
        b: b,
        c: d,
        material: triangle_mat.clone(),
    }));

    triangles.add(Box::new(Triangle {
        a: b,
        b: c,
        c: d,
        material: triangle_mat.clone(),
    }));
    triangles 
}

