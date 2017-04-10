use vector::Vec3;
use ray::Ray;
use intersection::{Intersectable, Intersection};
use reflectionmodel::ModifiedPhongModel;
use math::EPS;

#[cfg(test)]
use vector::VEC3_ZERO;
#[cfg(test)]
use textures::color::NoTexture;

pub struct Triangle<'a> {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub vec_ab: Vec3,
    pub vec_ac: Vec3,
    pub normal: Vec3,
    pub material: ModifiedPhongModel<'a>,
}

impl<'a> Triangle<'a> {

    fn new(a :Vec3, b :Vec3, c :Vec3, material :ModifiedPhongModel) -> Triangle {

        let vec_ab :Vec3 = Vec3::fromto(&a,&b);
        let vec_ac :Vec3 = Vec3::fromto(&a,&c);
        // TODO: normal in correct direction?
        let mut normal = vec_ab.cross(&vec_ac);
        normal.normalize();

        // fails for degenerate triangles
        assert!(normal.length() - 1.0 < EPS && normal.length() - 1.0 > -EPS);

        Triangle {
            a: a, b: b, c: c,

            vec_ab: vec_ab,
            vec_ac: vec_ac,

            normal: normal,
            material: material
        }
    }

}

impl<'a> Intersectable for Triangle<'a> {

    fn intersect(&self, ray :&Ray) -> Option<Intersection> {
        let p = ray.direction.cross(&self.vec_ac);
        let det = self.vec_ab.dot(&p);

        if det < EPS && det > EPS {
            return None;
        }
        let inv_det = 1.0 / det;

        let dist = ray.origin.sub(&self.a);

        let u = dist.dot(&p) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = dist.cross(&self.vec_ab);

        let v = ray.direction.dot(&q) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = self.vec_ac.dot(&q) * inv_det;

        if t > EPS {
            let isect = Intersection {
                t: t,
                normal: self.normal,
                material: &self.material,
                ray: *ray
            };

            return Some(isect);
        }

        None
    }

    fn get_material(&self) -> &ModifiedPhongModel {
        &self.material
    }

    fn reduce_to_point(&self) -> &Vec3 {
        &self.a
    }

    fn reduce_emission(&self) -> Vec3 {
        self.material.emission.map_texture(&self.a)
    }
}

#[cfg(test)]
const TEST_MATERIAL :ModifiedPhongModel = ModifiedPhongModel {
    emission:     &NoTexture { color: VEC3_ZERO },
    k_specular:   &NoTexture { color: VEC3_ZERO },
    k_diffus:     &NoTexture { color: VEC3_ZERO },
    k_ambient:    &NoTexture { color: VEC3_ZERO },
    k_t:          &NoTexture { color: VEC3_ZERO },

    phong_exponent: 1.0,
    ior: 0.0,
};

#[test]
fn triangle_intersection() {
    let triangle = Triangle::new(
        Vec3::new(1.0,1.0,0.0),
        Vec3::new(0.0,0.0,0.0),
        Vec3::new(0.0,1.0,0.0),
        TEST_MATERIAL
    );

    let ray = Ray::new(
        Vec3::new(0.25, 0.25, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
    );

    assert!(triangle.intersect(&ray).unwrap().t - 1.0 < EPS);
}

#[test]
fn triangle_no_intersection() {
    let triangle = Triangle::new(
        Vec3::new(1.0,1.0,0.0),
        Vec3::new(0.0,0.0,0.0),
        Vec3::new(0.0,1.0,0.0),
        TEST_MATERIAL
    );

    let ray = Ray::new(
        Vec3::new(0.25, 0.25, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    assert!(triangle.intersect(&ray).is_none());
}
