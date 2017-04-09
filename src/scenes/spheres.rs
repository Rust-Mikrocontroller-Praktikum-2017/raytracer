use scene::Scene;
use intersectables::sphere::Sphere;
use vector::{Vec3, VEC3_ZERO};
use reflectionmodel::ModifiedPhongModel;
use textures::color::NoTexture;

pub const SCENE_SPHERE: Scene = Scene {
    objects: &[
        &Sphere {
            center: Vec3 {x: 50.0, y: 50.0, z: 50.0},
            radius: 50.0,

            material: ModifiedPhongModel {
                emission:     &NoTexture { color: VEC3_ZERO },
                k_specular:   &NoTexture { color: VEC3_ZERO },
                k_diffus:     &NoTexture { color: Vec3 { x: 0.50, y: 0.50, z: 0.00 } },
                k_ambient:    &NoTexture { color: Vec3 { x: 0.50, y: 0.50, z: 0.00 } },
                k_t:          &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Sphere {
            center: Vec3 {x: 0.0, y: -100.0, z: 0.0},
            radius: 40.0,
            material: ModifiedPhongModel {
                emission:     &NoTexture { color: VEC3_ZERO },
                k_specular:   &NoTexture { color: Vec3 { x: 0.5, y: 0.5, z: 0.5 } },
                k_diffus:     &NoTexture { color: Vec3 { x: 0.5, y: 0.0, z: 0.0 } },
                k_ambient:    &NoTexture { color: Vec3 { x: 0.5, y: 0.0, z: 0.0 } },
                k_t:          &NoTexture { color: VEC3_ZERO },

                phong_exponent: 4.0,
                ior: 0.0,
            }
        },
        &Sphere {
            center: Vec3 {x: -100.0, y: -100.0, z: 10.0},
            radius: 0.0,
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: Vec3 { x: 1.0e4, y: 1.0e4, z: 1.0e4 } },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &NoTexture { color: VEC3_ZERO },
                k_ambient:      &NoTexture { color: VEC3_ZERO },
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
    ]
};
