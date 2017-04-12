use vector::Vec3;
use ray::Ray;
use intersection::{Intersectable, Intersection};
use reflectionmodel::ModifiedPhongModel;
use math::{max, EPS};

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

    pub fn new(a :Vec3, b :Vec3, c :Vec3, material :ModifiedPhongModel) -> Triangle {

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

    fn reduce_to_point(&self) -> Vec3 {
        let a_len = self.c.sub(&self.b).length();
        let b_len = self.vec_ac.length();
        let c_len = self.vec_ab.length();
        let circum = a_len+b_len+c_len;
        let center = self.a.mult(a_len / circum)
            .add(&self.b.mult(b_len / circum))
            .add(&self.c.mult(c_len / circum));
        center
    }

    fn reduce_emission(&self) -> Vec3 {
        self.material.emission.map_texture(&self.a, self)
    }

    fn maximum_expansion(&self, center: &Vec3) -> f32 {
        let dist_a = self.a.sub(center).length();
        let dist_b = self.b.sub(center).length();
        let dist_c = self.c.sub(center).length();
        max(max(dist_a, dist_b), dist_c)
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
