use crate::math::Ray;
use crate::math::Vec3;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, up: Vec3, fovy: f32, aspect_ratio: f32) -> Camera {
        let theta = fovy.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (position - target).normalized();
        let u = (Vec3::cross(&up, &w)).normalized();
        let v = Vec3::cross(&w, &u);

        let origin = position;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin,
        }
    }
}
