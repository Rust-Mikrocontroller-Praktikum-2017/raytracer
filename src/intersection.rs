use ray::Ray;
use vector::Vec3;

pub trait Intersectable {
    //fn intersects_enveloping_body(&self, &Ray) -> bool;
    fn intersect(&self, &Ray) -> &mut Option<Intersection>;
    //fn add_to_aabb(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub t :f64
}

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Intersectable for Sphere {
    fn intersect(&self, ray :&Ray) -> &mut Option<Intersection> {
        &mut None
    }
}
