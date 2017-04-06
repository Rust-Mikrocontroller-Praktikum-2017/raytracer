use ray::Ray;
use vector::Vec3;
use math::sqrt;
use lcd::Color;

pub trait Intersectable {
    //fn intersects_enveloping_body(&self, &Ray) -> bool;
    fn intersect(&self, &Ray) -> &mut Option<Intersection>;
    //fn add_to_aabb(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub t :f64,
    pub normal :Vec3,
    pub material : Color,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Color,
}

impl Sphere {
    fn makeIntersection(&self, t :f64, ray :&Ray) -> Intersection {
        let normal = self.center.sub(&(ray.origin.add(&(ray.direction.mult(t)))));
        normal.normalize();

        Intersection {
            t: t,
            normal: normal,
            material: self.material
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray :&Ray) -> &mut Option<Intersection> {
        let dist = ray.origin.sub(&self.center);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&dist);
        let c = dist.dot(&dist) - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            return &mut None;
        } 

        let mut t;

        let t1 = (-b - sqrt(discriminant)) / (2.0*a);

        if discriminant == 0.0 {
            return &mut Some(self.makeIntersection(t1,ray));
        }

        let t2 = (-b + sqrt(discriminant)) / (2.0*a);

        if t1 < 0.0 && t2 > 0.0 {
            return &mut Some(self.makeIntersection(t2,ray));
        }

        return &mut Some(self.makeIntersection(t1,ray));
    }
}
