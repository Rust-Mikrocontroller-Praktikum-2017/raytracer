use core::intrinsics;

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

impl Vec3 {
    pub fn new(x :f64, y :f64, z :f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        unsafe{intrinsics::sqrtf64(self.length_squared())}
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

    pub fn sub(&self, other :&Self) -> Self {
        Vec3 { 
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn add(&self, other :&Self) -> Self {
        Vec3 { 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn mult(&self, other :f64) -> Self {
        Vec3 { 
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
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

    pub fn cross(&self, other :&Self) -> Vec3 {
        Vec3 {
            x: self.y*other.z - other.z*self.y,
            y: self.z*other.x - other.x*self.z,
            z: self.x*other.y - other.y*self.x,
        }
    }

    pub fn reflect(&self, n :&Self) -> Self {
        let cos = self.dot(n);
        let mut reflected = n.clone();

        reflected.inplace_mult(2.0*cos).inplace_sub(self);
        reflected
    }
}

#[test]
fn can_compute_cross_product() {
    let vec1 = Vec3::new(1.0,2.0,3.0);
    let vec2 = Vec3::new(-7.0,8.0,9.0);
    let res = vec1.cross(vec2);

    assert_eq!(res.x,  -6.0);
    assert_eq!(res.y, -30.0);
    assert_eq!(res.z,  22.0);
}

#[test]
fn can_reflect_vector() {
    let mut vec = Vec3::new(-1.0,1.0,0.0);
    vec.normalize();
    let mut normal = Vec3::new(0.0,1.0,0.0);
    normal.normalize();

    let reflected = vec.reflect(&normal);

    assert_eq!(reflected.x, 1.0);
    assert_eq!(reflected.y, 1.0);
    assert_eq!(reflected.z, 0.0);
}

impl Vec2 {
    pub fn new(u :f64, v :f64) -> Vec2 {
        Vec2 {
            u: u,
            v: v,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.u * self.u + self.v * self.v
    }

    pub fn length(&self) -> f64 {
        unsafe{intrinsics::sqrtf64(self.length_squared())}
    }

    pub fn normalize(&mut self) -> &mut Self {
        let length = self.length();

        self.u = self.u / length;
        self.v = self.v / length;

        self
    }

    pub fn inplace_add(&mut self, other :&Self) -> &mut Self {
        self.u = self.u + other.u;
        self.v = self.v + other.v;
        self
    }

    pub fn inplace_sub(&mut self, other :&Self) -> &mut Self {
        self.u = self.u - other.u;
        self.v = self.v - other.v;
        self
    }

    pub fn sub(&self, other :&Self) -> Self {
        Vec2 { 
            u: self.u - other.u,
            v: self.v - other.v,
        }
    }

    pub fn add(&self, other :&Self) -> Self {
        Vec2 { 
            u: self.u + other.u,
            v: self.v + other.v,
        }
    }

    pub fn mult(&self, other :f64) -> Self {
        Vec2 { 
            u: self.u * other,
            v: self.v * other,
        }
    }

    pub fn mult_vec(&self, other :&Self) -> Self {
        Vec2 { 
            u: self.u * other.u,
            v: self.v * other.v,
        }
    }

    pub fn inplace_mult(&mut self, other :f64) -> &mut Self {
        self.u = self.u * other;
        self.v = self.v * other;
        self
    }
}
