use crate::material::Constant;
use crate::material::Material;
use crate::math::Color;
use crate::math::Ray;
use crate::math::Vec3;

use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    // Intersection point in world coordinates.
    pub point: Vec3,
    // Normal at intersection point in world coordinates.
    pub normal: Vec3,
    // Ray march param.
    pub t: f32,
    // Set to true if the ray hit the front facing.
    pub front_face: bool,

    pub material: Arc<dyn Material>,
}

// All the objects we can intersect.
pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<dyn Material>,
}

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    material: Arc<dyn Material>,
}

// See: https://bennetthardwick.com/blog/dont-use-boxed-trait-objects-for-struct-internals/
pub struct IntersectableList<I: Intersectable> {
    objects: Vec<I>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            material: Arc::new(Constant {
                color: Color {
                    r: 1.0,
                    g: 0.0,
                    b: 1.0,
                },
            }),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = outward_normal;
        if !self.front_face {
            self.normal *= -1.0
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.mag_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        hit.t = root;
        hit.point = ray.at(root);
        let normal = (hit.point - self.center) / self.radius;
        hit.set_face_normal(ray, normal);
        hit.material = self.material.clone();
        true
    }
}

impl Triangle {
    pub fn normal(&self) -> Vec3 {
        let u = (self.b - self.a).normalized();
        let v = (self.c - self.a).normalized();
        Vec3::cross(&u, &v)
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut HitRecord) -> bool {
        let u = self.b - self.a;
        let v = self.c - self.a;
        let n = Vec3::cross(&u, &v);

        // Step 1 -- find intersection point.
        // Check if plane and ray are parallel.
        let n_dot_ray_dir = Vec3::dot(&n, &ray.direction);
        if n_dot_ray_dir.abs() < f32::EPSILON {
            return false;
        }

        let d = Vec3::dot(&n, &self.a);
        // Check if t is within valid bounds.
        let t = (Vec3::dot(&n, &ray.origin) + d) / n_dot_ray_dir;
        if t < t_min || t > t_max {
            return false;
        }

        // Calculate intersection point.
        let point = ray.origin + ray.direction * t;

        // Step 2 -- inside-out test.
        let c = Vec3::cross(&n, &(point - u));
        if Vec3::dot(&n, &c) < 0.0 {
            return false;
        }

        let c = Vec3::cross(&n, &(point - v));
        if Vec3::dot(&n, &c) < 0.0 {
            return false;
        }

        let c = Vec3::cross(&n, &(point - self.a - self.c));
        if Vec3::dot(&n, &c) < 0.0 {
            return false;
        }

        hit.t = t;
        hit.front_face = Vec3::dot(&hit.normal, &ray.direction) < 0.0;
        hit.point = point;
        hit.set_face_normal(ray, self.normal());
        hit.material = self.material.clone();

        true
    }
}

impl<I: Intersectable> IntersectableList<I> {
    pub fn new() -> IntersectableList<I> {
        IntersectableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: I) {
        self.objects.push(object);
    }
}

impl<I: Intersectable> Intersectable for IntersectableList<I> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut HitRecord) -> bool {
        let mut any_hit = false;
        let mut closest = t_max;
        let mut record: HitRecord = HitRecord::new();
        for obj in self.objects.iter() {
            if obj.intersect(&ray, t_min, closest, &mut record) {
                any_hit = true;
                closest = record.t;
                *hit = record.clone();
            }
        }
        any_hit
    }
}
