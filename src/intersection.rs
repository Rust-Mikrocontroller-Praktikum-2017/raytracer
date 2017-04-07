use ray::Ray;
use vector::Vec3;
use math::sqrt;
use lcd::Color;
use reflectionmodel::ModifiedPhongModel;

pub trait Intersectable {
    //fn intersects_enveloping_body(&self, &Ray) -> bool;
    fn intersect(&self, &Ray) -> Option<Intersection>;
    // TODO: use generics or something to make the material swappable
    fn get_material(&self) -> &ModifiedPhongModel;
    //fn add_to_aabb(&self);
    fn reduce_to_point(&self) -> &Vec3;
}


#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    pub t :f32,
    pub normal :Vec3,
    pub material : &'a ModifiedPhongModel,
    pub ray : Ray
}

impl<'a> Intersection<'a> {
    pub fn get_position(&self) -> Vec3 {
        *self.ray.direction.mult(self.t).inplace_add(&self.ray.origin)
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: ModifiedPhongModel,
}

impl Sphere {
    fn make_intersection(&self, t :f32, ray :&Ray) -> Intersection {
        let mut normal = self.center.sub(&(ray.origin.add(&(ray.direction.mult(t)))));
        normal.normalize();

        Intersection {
            t: t,
            normal: normal,
            material: &self.material,
            ray: ray.clone()
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

    fn get_material(&self) -> &ModifiedPhongModel {
        &self.material
    }

    fn reduce_to_point(&self) -> &Vec3 {
        &self.center
    }
}
