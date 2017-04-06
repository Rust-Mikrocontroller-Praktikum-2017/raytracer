use vector::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin :Vec3, direction :Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction
        }
    }
}
