use scene::Scene;
use intersection::Sphere;
use vector::{Vec3, VEC3_ZERO, VEC3_ONE};
use reflectionmodel::ModifiedPhongModel;

pub static SCENE_SPHERE: Scene = Scene {
    objects: &[
        &Sphere {
            center: Vec3 {x: 1.0, y: 1.0, z: 1.0},
            radius: 1.0,
            material: ModifiedPhongModel {
                emission: VEC3_ZERO,
                k_specular: VEC3_ZERO,
                k_diffus: Vec3 { x: 1.00, y: 1.00, z: 0.00 },
                k_ambient: Vec3 { x: 1.00, y: 1.00, z: 0.00 },
                phong_exponent: 1.0,
                k_t: VEC3_ZERO,
                ior: 0.0,
                transmitting: false
            }
        },
        &Sphere {
            center: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            radius: 1.0,
            material: ModifiedPhongModel {
                emission: VEC3_ZERO,
                k_specular: VEC3_ZERO,
                k_diffus: Vec3 { x: 0.00, y: 1.00, z: 0.00 },
                k_ambient: Vec3 { x: 0.00, y: 1.00, z: 0.00 },
                phong_exponent: 1.0,
                k_t: VEC3_ZERO,
                ior: 0.0,
                transmitting: false
            }
        },
        &Sphere {
            center: Vec3 {x: 5.0, y: 5.0, z: 5.0},
            radius: 0.5,
            material: ModifiedPhongModel {
                emission: VEC3_ONE,
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
