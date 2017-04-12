use vector::Vec3;
use ray::Ray;
use intersection::{Intersectable, Intersection};
use reflectionmodel::ModifiedPhongModel;
use math::sqrt;

#[cfg(test)]
use vector::VEC3_ZERO;
#[cfg(test)]
use textures::color::NoTexture;
#[cfg(test)]
use math::EPS;


pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f32,
    pub material: ModifiedPhongModel<'a>,
}

impl<'a> Sphere<'a> {
    fn make_intersection(&self, t :f32, ray :Ray) -> Intersection {
        let mut normal = ray.get(t).sub(&self.center);
        normal.normalize();

        Intersection {
            t: t,
            normal: normal,
            material: &self.material,
            ray: ray
        }
    }
}

impl<'a> Intersectable for Sphere<'a> {

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
            return Some(self.make_intersection(t1,*ray));
        }

        let t2 = (-b + sqrt(discriminant)) / (2.0*a);

        if t1 < 0.0 && t2 > 0.0 {
            return Some(self.make_intersection(t2,*ray));
        }

        Some(self.make_intersection(t1,*ray))
    }

    fn get_material(&self) -> &ModifiedPhongModel {
        &self.material
    }

    fn reduce_to_point(&self) -> Vec3 {
        self.center
    }

    fn reduce_emission(&self) -> Vec3 {
        let mut surface_point = Vec3::new(0.0, 0.0, self.radius);
        surface_point.inplace_add(&self.center);
        self.material.emission.map_texture(&surface_point, self)
    }

    fn maximum_expansion(&self, _center: &Vec3) -> f32 {
        self.radius
    }

    //fn has_surface_area(&self) -> bool {
        //self.radius > 0.0
    //}

    // TODO: this should only sample the potentially visible hemisphere
    // currently the whole sphere is sampled
    //fn sample_area_light(&self) -> Vec3 {
        //let random_1 = u32_to_f32(RANDOM.as_mut_ref().next_u32());
        //let random_2 = u32_to_f32(RANDOM.as_mut_ref().next_u32());

        //let theta = arccos(1-random_1);
        //let phi = TWOPI * random_2;

        //let postive_hemisphere = Vec3 {
            //x: r * sin(theta) * cos(phi),
            //y: r * sin(theta) * sin(phi),
            //z: r * cos(theta)
        //};
    //}
}

fn u32_to_f32(a :u32) -> f32 {
    (a as f32) / (u32::max_value() as f32)
}

#[cfg(test)]
const TEST_SPHERE : Sphere = Sphere {
    center: Vec3 {x: 0.0, y: 0.0, z: 0.0},
    radius: 1.0,
    material: ModifiedPhongModel {
        emission:     &NoTexture { color: VEC3_ZERO },
        k_specular:   &NoTexture { color: VEC3_ZERO },
        k_diffus:     &NoTexture { color: Vec3 { x: 0.50, y: 0.50, z: 0.00 } },
        k_ambient:    &NoTexture { color: Vec3 { x: 0.50, y: 0.50, z: 0.00 } },
        k_t:          &NoTexture { color: VEC3_ZERO },

        phong_exponent: 1.0,
        ior: 0.0,
    }
};

#[test]
fn sphere_no_intersection_works() {

    //No intersection
    let ray = &Ray::new(Vec3::new(-2.0,0.0,0.0), Vec3::new(0.0,1.0,0.0));
    assert!(TEST_SPHERE.intersect(ray).is_none());
}

#[test]
fn sphere_one_intersection_works() {
    use math::EPS;

    //One intersection
    let ray = &Ray::new(Vec3::new(-1.0,1.0,0.0), Vec3::new(1.0,0.0,0.0));
    assert!(TEST_SPHERE.intersect(ray).unwrap().t - 1.0 < EPS);
}

#[test]
fn sphere_two_intersection_works() {
    //Two intersections in front of camera
    let ray = &Ray::new(Vec3::new(-2.0,0.0,0.0), Vec3::new(1.0,0.0,0.0));
    assert!(TEST_SPHERE.intersect(ray).unwrap().t - 1.0 < EPS);
}

#[test]
fn sphere_intersection_inside_works() {
    //Two intersections, camera in sphere
    let ray = &Ray::new(Vec3::new(0.0,0.0,0.0), Vec3::new(1.0,0.0,0.0));
    assert!(TEST_SPHERE.intersect(ray).unwrap().t - 1.0 < EPS);
}

#[test]
fn sphere_intersection_behind_works() {
    //Intersections behind camera
    let ray = &Ray::new(Vec3::new(2.0,0.0,0.0), Vec3::new(1.0,0.0,0.0));
    assert!(TEST_SPHERE.intersect(ray).unwrap().t < 0.0);
}
