use ray::Ray;
use vector::Vec3;
use reflectionmodel::ModifiedPhongModel;

pub trait Intersectable {
    //fn intersects_enveloping_body(&self, &Ray) -> bool;
    fn intersect(&self, &Ray) -> Option<Intersection>;
    fn get_material(&self) -> &ModifiedPhongModel;
    //fn add_to_aabb(&self);
    /// Reduces light sources with a non-zero surface area to point lights for shading models that
    /// do only support point lights.
    fn reduce_to_point(&self) -> Vec3;
    /// emission of the reduced point light source. Area lights with a non zero emission at any
    /// surface point MUST return a non-zero emission value for point lights.
    fn reduce_emission(&self) -> Vec3;
    /// radius of a sphere enveloping the whole intersectable.
    fn maximum_expansion(&self, center: &Vec3) -> f32;
    ///// true if the light source is has no surface area
    //fn is_point_light(&self) -> bool;
    ///// returns a random point on the visible area of
    ///// the object
    //fn sample_area_light(&self) -> Vec3;
    fn is_light(&self) -> bool {
        self.reduce_emission().max_norm() > 0.0
    }
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

