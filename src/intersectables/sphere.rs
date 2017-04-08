use vector::Vec3;
use ray::Ray;
use intersection::{Intersectable, Intersection};
use reflectionmodel::ModifiedPhongModel;
use math::sqrt;


pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: ModifiedPhongModel,
}

impl Sphere {
    fn make_intersection(&self, t :f32, ray :Ray) -> Intersection {
        let mut normal = self.center.sub(&(ray.origin.add(&(ray.direction.mult(t)))));
        normal.normalize();

        Intersection {
            t: t,
            normal: normal,
            material: &self.material,
            ray: ray
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

    fn reduce_to_point(&self) -> &Vec3 {
        &self.center
    }
}

#[cfg(test)]
fn create_sphere() -> Sphere {
    use vector::{VEC3_ONE, VEC3_ZERO};

    Sphere {
        center: Vec3 {x: 0.0, y: 0.0, z: 0.0},
        radius: 1.0,
        material: ModifiedPhongModel {
            emission: VEC3_ZERO,
            k_specular: VEC3_ONE,
            k_diffus: Vec3 { x: 1.00, y: 1.00, z: 0.00 },
            k_ambient: Vec3 { x: 0.25, y: 0.25, z: 0.00 },
            phong_exponent: 1.0,
            k_t: VEC3_ZERO,
            ior: 0.0,
            transmitting: false
        }
    }
}

#[test]
fn sphere_no_intersection_works() {
    let sphere = create_sphere();

    //No intersection
    let ray = &Ray::new(Vec3::new(-2.0,0.0,0.0), Vec3::new(0.0,1.0,0.0));
    assert!(sphere.intersect(ray).is_none());
}

#[test]
fn sphere_one_intersection_works() {
    use math::EPS;
    let sphere = create_sphere();

    //One intersection
    let ray = &Ray::new(Vec3::new(-1.0,1.0,0.0), Vec3::new(1.0,0.0,0.0));
    assert!(sphere.intersect(ray).unwrap().t - 1.0 < EPS);
}

#[test]
fn sphere_two_intersection_works() {
    use math::EPS;
    let sphere = create_sphere();

    //Two intersections in front of camera
    let ray = &Ray::new(Vec3::new(-2.0,0.0,0.0), Vec3::new(1.0,0.0,0.0));
    assert!(sphere.intersect(ray).unwrap().t - 1.0 < EPS);
}

#[test]
fn sphere_intersection_inside_works() {
    use math::EPS;
    let sphere = create_sphere();

    //Two intersections, camera in sphere
    let ray = &Ray::new(Vec3::new(0.0,0.0,0.0), Vec3::new(1.0,0.0,0.0));
    assert!(sphere.intersect(ray).unwrap().t - 1.0 < EPS);
}

#[test]
fn sphere_intersection_behind_works() {
    let sphere = create_sphere();

    //Intersections behind camera
    let ray = &Ray::new(Vec3::new(2.0,0.0,0.0), Vec3::new(1.0,0.0,0.0));
    assert!(sphere.intersect(ray).unwrap().t < 0.0);
}
