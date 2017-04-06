use std::ops::{Add,Div,Mul};

fn main() {

}

#[derive(Debug)]
pub struct Vec3<T : Add + Div + Mul> {
    x: T,
    y: T,
    z: T,
}

impl<T> Add for Vec3<T> where T: Add<Output=T> + Div + Mul {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
            z: (self.z + rhs.z),
        }
    }
}

impl<T : Add<Output=T> + Div + Mul> Vec3<T> {
    fn new(x :T, y :T, z :T) {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn lengthSquared(&self) {
        self.x * self.x + self.y * self.y + self.z *self.z
    }

    pub fn length(&self) {
        self.lengthSquared().sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();

        self.x = self.x / length;
        self.y = self.y / length;
        self.y = self.y / length;

        self
    }

    pub fn reflect(&self, n :&Vec3<T>) {

    }
}
