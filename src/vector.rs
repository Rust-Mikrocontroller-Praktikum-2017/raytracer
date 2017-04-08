use math::{min, max, sqrt, EPS};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub u: f32,
    pub v: f32,
}

pub const VEC3_ZERO :Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
pub const VEC3_ONE  :Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

impl Vec3 {
    pub fn fromto(a :&Vec3, b :&Vec3) -> Vec3 {
        b.sub(a)
    }

    pub fn zero() -> Vec3 {
        Self::new(0.0,0.0,0.0)
    }

    pub fn one() -> Vec3 {
        Self::new(1.0,1.0,1.0)
    }

    pub fn new(x :f32, y :f32, z :f32) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        sqrt(self.length_squared())
    }

    pub fn max_norm(&self) -> f32 {
        max(max(self.x, self.y), self.z)
    }

    pub fn normalize(&mut self) -> &mut Self {
        let length = self.length();

        self.x /= length;
        self.y /= length;
        self.z /= length;

        self
    }

    pub fn inplace_add(&mut self, other :&Self) -> &mut Self {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }

    pub fn inplace_sub(&mut self, other :&Self) -> &mut Self {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
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

    pub fn mult(&self, other :f32) -> Self {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }

    pub fn div(&self, other: f32) -> Self {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }

    pub fn mult_vec(&self, other :&Vec3) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn inplace_mult_vec(&mut self, other :&Vec3) -> &mut Self {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self
    }

    pub fn inplace_mult(&mut self, other :f32) -> &mut Self {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self
    }

    pub fn dot(&self, other :&Self) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn cross(&self, other :&Self) -> Vec3 {
        Vec3 {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }

    pub fn reflect(&self, n :&Self) -> Self {
        let cos = self.dot(n);
        self.sub(&n.mult(2.0*cos))
    }

    pub fn refract(&self, n: &Self, ior: f32, inside: bool) -> Option<Vec3> {
        assert!(self.length_squared() - 1.0 < EPS);
        // self*n = cos(theta_1)
        let (ior1, ior2, self_dot_n, nn): (f32,f32,f32,Vec3) = if !inside {
            (1.0, ior, self.dot(n), *n)
        } else {
            (ior, 1.0, self.dot(n), n.mult(-1.0))
        };

        let ratio = ior1 / ior2;
        let discriminant = 1.0 - ratio*ratio * (1.0 - self_dot_n*self_dot_n);

        if discriminant < 0.0 {
            None
        } else {
            Some(self.mult(-ratio).add(&nn.mult(self_dot_n*ratio-sqrt(discriminant))))
        }
    }

    pub fn min(a :&Vec3, b :&Vec3) -> Vec3 {
        Vec3 {
            x: min(a.x, b.x),
            y: min(a.y, b.y),
            z: min(a.z, b.z)
        }
    }

    pub fn max(a :&Vec3, b :&Vec3) -> Vec3 {
        Vec3 {
            x: max(a.x, b.x),
            y: max(a.y, b.y),
            z: max(a.z, b.z)
        }
    }
}

#[test]
fn can_compute_cross_product() {
    let vec1 = Vec3::new(1.0,2.0,3.0);
    let vec2 = Vec3::new(-7.0,8.0,9.0);
    let res = vec1.cross(&vec2);

    assert_eq!(res.x,  -6.0);
    assert_eq!(res.y, -30.0);
    assert_eq!(res.z,  22.0);
}

#[test]
fn can_reflect_vector() {
    use math::EPS;
    let mut vec = Vec3::new(-1.0,1.0,0.0);
    vec.normalize();
    let mut normal = Vec3::new(0.0,1.0,0.0);
    normal.normalize();

    let reflected = vec.reflect(&normal);

    assert!(reflected.x - sqrt(2.0) < EPS);
    assert!(reflected.y - sqrt(2.0) < EPS);
    assert_eq!(reflected.z, 0.0);
}

impl Vec2 {
    pub fn new(u :f32, v :f32) -> Vec2 {
        Vec2 {
            u: u,
            v: v,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.u * self.u + self.v * self.v
    }

    pub fn length(&self) -> f32 {
        sqrt(self.length_squared())
    }

    pub fn normalize(&mut self) -> &mut Self {
        let length = self.length();

        self.u /= length;
        self.v /= length;

        self
    }

    pub fn inplace_add(&mut self, other :&Self) -> &mut Self {
        self.u += other.u;
        self.v += other.v;
        self
    }

    pub fn inplace_sub(&mut self, other :&Self) -> &mut Self {
        self.u -= other.u;
        self.v -= other.v;
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

    pub fn mult(&self, other :f32) -> Self {
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

    pub fn inplace_mult(&mut self, other :f32) -> &mut Self {
        self.u *= other;
        self.v *= other;
        self
    }
}
