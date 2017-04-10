use ray::Ray;
use vector::Vec3;
use reflectionmodel::ModifiedPhongModel;

pub trait Intersectable {
    //fn intersects_enveloping_body(&self, &Ray) -> bool;
    fn intersect(&self, &Ray) -> Option<Intersection>;
    // TODO: use generics or something to make the material swappable
    fn get_material(&self) -> &ModifiedPhongModel;
    //fn add_to_aabb(&self);
    /// Reduces light sources with a non-zero surface area to point lights
    /// for shading models that do only support point lights.
    fn reduce_to_point(&self) -> &Vec3;
    /// emission of the reduced point light source
    fn reduce_emission(&self) -> Vec3;
}


#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub t :f32,
    pub normal :Vec3,
    pub material : &'a ModifiedPhongModel<'a>,
    pub ray : Ray
}

impl<'a> Intersection<'a> {
    pub fn get_position(&self) -> Vec3 {
        *self.ray.direction.mult(self.t).inplace_add(&self.ray.origin)
    }
}

