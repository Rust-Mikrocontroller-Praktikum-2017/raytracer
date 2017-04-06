use std::ops::{Add,Div,Mul,Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub u: f64,
    pub v: f64,
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            u: self.u * rhs.u,
            v: self.v * rhs.v,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Vec2 {
    pub fn new(u :f64, v :f64) -> Vec2 {
        Vec2 {
            u: u,
            v: v,
        }
    }
}

impl Vec3 {
    pub fn new(x :f64, y :f64, z :f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z *self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&mut self) -> &mut Self {
        let length = self.length();

        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;

        self
    }

    pub fn inplace_add(&mut self, other :&Self) -> &mut Self {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
        self
    }

    pub fn inplace_sub(&mut self, other :&Self) -> &mut Self {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
        self
    }

    pub fn inplace_mult(&mut self, other :f64) -> &mut Self {
        self.x = self.x * other;
        self.y = self.y * other;
        self.z = self.z * other;
        self
    }

    pub fn dot(&self, other :&Self) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn reflect(&self, n :&Self) -> Self {
        let cos = self.dot(n);
        let mut reflected = n.clone();


        //*(reflected.inplace_mult(2.0*cos).inplace_sub(self))
        
        //let ret = reflected.inplace_mult(2.0*cos).inplace_sub(self);
        //*ret

        reflected.inplace_mult(2.0*cos).inplace_sub(self);
        reflected
    }
}
