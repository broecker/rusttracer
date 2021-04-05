
use cgmath::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {

    pub fn at(&self, t: f32) -> cgmath::Vector3 {
        return self.origin + self.direction * t;
    }

}
