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
pub trait Intersectable : Sync {
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
    pub material: Arc<dyn Material>,
}

pub struct Intersectables {
    objects: Vec<Box<dyn Intersectable>>,
}

impl Intersectable for Intersectables {
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

impl Intersectables {
    pub fn new() -> Intersectables {
        Intersectables {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Intersectable>) {
        self.objects.push(object)
    }

    pub fn join(&mut self, other: Intersectables) {
        for obj in other.objects {
            self.add(obj);
        }
    }
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
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let p_vec = Vec3::cross(&ray.direction, &e2);
        let det = Vec3::dot(&e1, &p_vec);

        // Ray and triangle are parallel if close to 0.
        if det.abs() < f32::EPSILON {
            return false;
        }

        let inv_det = 1.0 / det;
        let t_vec = ray.origin - self.a;

        // Barycentric coord u.
        let u = Vec3::dot(&t_vec, &p_vec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return false;
        }

        // Barycentric coord v.
        let q_vec = Vec3::cross(&t_vec, &e1);
        let v = Vec3::dot(&ray.direction, &q_vec) * inv_det;
        if v < 0.0 || v > 1.0 {
            return false;
        }

        let t = Vec3::dot(&e2, &q_vec) * inv_det;
        if t < t_min || t > t_max {
            return false;        }


        hit.t = t;
        hit.front_face = Vec3::dot(&hit.normal, &ray.direction) < 0.0;
        hit.point = ray.origin + ray.direction * hit.t;
        hit.set_face_normal(ray, self.normal());
        hit.material = self.material.clone();

        true
    }
}
