use scene::Scene;
use intersectables::sphere::Sphere;
use vector::{Vec3, VEC3_ZERO};
use reflectionmodel::ModifiedPhongModel;

pub static SCENE_SPHERE: Scene = Scene {
    objects: &[
        &Sphere {
            center: Vec3 {x: 50.0, y: 50.0, z: 50.0},
            radius: 50.0,
            material: ModifiedPhongModel {
                emission: VEC3_ZERO,
                k_specular: VEC3_ZERO,
                k_diffus: Vec3 { x: 0.50, y: 0.50, z: 0.00 },
                k_ambient: Vec3 { x: 0.50, y: 0.50, z: 0.00 },
                phong_exponent: 1.0,
                k_t: VEC3_ZERO,
                ior: 0.0,
                transmitting: false
            }
        },
        &Sphere {
            center: Vec3 {x: 0.0, y: -100.0, z: 0.0},
            radius: 40.0,
            material: ModifiedPhongModel {
                emission: VEC3_ZERO,
                k_specular: Vec3 { x: 0.5, y: 0.5, z: 0.5 },
                k_diffus: Vec3 { x: 0.5, y: 0.0, z: 0.0 },
                k_ambient: Vec3 { x: 0.5, y: 0.0, z: 0.0 },
                phong_exponent: 4.0,
                k_t: VEC3_ZERO,
                ior: 0.0,
                transmitting: false
            }
        },
        &Sphere {
            center: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            radius: 50.0,
            material: ModifiedPhongModel {
                emission: VEC3_ZERO,
                k_specular: VEC3_ZERO,
                k_diffus: Vec3 { x: 0.00, y: 0.50, z: 0.00 },
                k_ambient: Vec3 { x: 0.00, y: 0.50, z: 0.00 },
                phong_exponent: 1.0,
                k_t: VEC3_ZERO,
                ior: 0.0,
                transmitting: false
            }
        },
        &Sphere {
            center: Vec3 {x: -100.0, y: -100.0, z: 10.0},
            radius: 0.0,
            material: ModifiedPhongModel {
                emission: Vec3 { x: 1.0e4, y: 1.0e4, z: 1.0e4 },
                k_specular: VEC3_ZERO,
                k_diffus: VEC3_ZERO,
                k_ambient: VEC3_ZERO,
                phong_exponent: 1.0,
                k_t: VEC3_ZERO,
                ior: 0.0,
                transmitting: false
            }
        },
    ]
};
