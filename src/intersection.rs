use ray::Ray;
use vector::Vec3;
use math::sqrt;
use lcd::Color;

pub trait Intersectable {
    //fn intersects_enveloping_body(&self, &Ray) -> bool;
    fn intersect(&self, &Ray) -> Option<Intersection>;
    //fn add_to_aabb(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub t :f32,
    pub normal :Vec3,
    pub material : Color,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Color,
}

impl Sphere {
    fn make_intersection(&self, t :f32, ray :&Ray) -> Intersection {
        let mut normal = self.center.sub(&(ray.origin.add(&(ray.direction.mult(t)))));
        normal.normalize();

        Intersection {
            t: t,
            normal: normal,
            material: self.material
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray :&Ray) -> Option<Intersection> {
        let dist = ray.origin.sub(&self.center);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&dist);
        let c = dist.dot(&dist) - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            return None;
        } 

        let t1 = (-b - sqrt(discriminant)) / (2.0*a);

        if discriminant == 0.0 {
            return Some(self.make_intersection(t1,ray));
        }

        let t2 = (-b + sqrt(discriminant)) / (2.0*a);

        if t1 < 0.0 && t2 > 0.0 {
            return Some(self.make_intersection(t2,ray));
        }

        return Some(self.make_intersection(t1,ray));
    }
}
