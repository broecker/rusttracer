mod bounds;
mod camera;
mod image;
mod intersection;
mod material;
mod math;
mod scene;

use math::Color;
use math::Ray;
use math::Vec3;

use rand::Rng;

use crate::intersection::{Intersectable, Intersectables};
use camera::Camera;
use intersection::HitRecord;

extern crate scoped_threadpool;
use scoped_threadpool::Pool;

struct RenderSettings {
    samples_per_pixel: u32,
    max_recursion_depth: u32,
    image_gamma: f32,
    render_threads: u32,
}

fn ray_color(
    ray: &Ray,
    world: &Intersectables,
    depth: u32,
) -> Color {
    let mut hit_record = HitRecord::new();

    if depth <= 0 {
        return Color::black();
    }

    if world.intersect(ray, 0.0001, 10000.0, &mut hit_record) {
        let mut scattered = Ray {
            origin: Vec3::zero(),
            direction: Vec3::zero(),
        };
        let mut attenuation = Color::black();

        if hit_record
            .material
            .scatter(ray, &hit_record, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, &world, depth - 1);
        }

        return Color::black();
    }

    // Background color
    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    let blueish = Color {
        r: 0.5,
        g: 0.7,
        b: 1.0,
    };

    Color::lerp(&white, &blueish, t)
}

fn trace(
    image: &mut image::Image,
    camera: &Camera,
    world: &Intersectables,
    render_settings: &RenderSettings,
) {
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
                    for i in 0..tile.image.width() {
                        let mut color = Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                        };
                        for _sample in 0..render_settings.samples_per_pixel {
                            let coord = tile.tile_to_image_coordinates(i, j);

                            let u =
                                (coord.0 as f32 + rng.gen_range(0.0..1.0)) / (image_w as f32 - 1.0);
                            let v = 1.0
                                - (coord.1 as f32 + rng.gen_range(0.0..1.0))
                                    / (image_h as f32 - 1.0);

                            let ray = camera.get_ray(u, v);
                            color += ray_color(
                                &ray,
                                &world,
                                render_settings.max_recursion_depth,
                            );
                        }

                        tile.image.put_pixel(
                            i,
                            j,
                            color / render_settings.samples_per_pixel as f32,
                        );
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

    let render_settings = RenderSettings {
        samples_per_pixel: 1,
        max_recursion_depth: 5,
        image_gamma: 2.0,
        render_threads: 12,
    };

    // World
    let mut world = scene::make_random_sphere_scene();
    world.join(scene::make_ground());
    //world.join(scene::load_ply(String::from("./data/bun_zipper.ply")));

    world = scene::load_ply(String::from("./data/bun_zipper.ply"));
    let bbox = world.get_aabb();
    println!("World {:?} -> {:?}", bbox.min, bbox.max);




    // Camera
    let camera = Camera::new(
        Vec3 {
            x: 13.0,
            y: 2.0,
            z: 3.0,
        },
        Vec3::zero(),
        Vec3::up(),
        60.0,
        image.aspect_ratio(),
    );

    trace(&mut image, &camera, &world, &render_settings);

    // File output.
    image.gamma_correct(render_settings.image_gamma);
    image.write_ppm("output.ppm".to_string());
}
