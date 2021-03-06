use scene::Scene;
use intersectables::sphere::Sphere;
use vector::{Vec3, VEC3_ZERO};
use reflectionmodel::ModifiedPhongModel;
use textures::color::{NoTexture};

pub const SCENE_ENVMAP_DEMO: Scene = Scene {
    objects: &[
        &Sphere {
            center: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            radius: 50.0,

            material: ModifiedPhongModel {
                emission:     &NoTexture { color: VEC3_ZERO },
                k_specular:   &NoTexture { color: Vec3 { x: 0.9, y: 0.9, z: 0.9 } },
                k_ambient:   &NoTexture { color: VEC3_ZERO },
                k_diffus:     &NoTexture { color: Vec3 { x: 0.50, y: 0.50, z: 0.50 } },
                k_t:          &NoTexture { color: VEC3_ZERO },

                phong_exponent: 3.0,
                ior: 0.0,
            }
        },
        &Sphere {
            center: Vec3 {x: 200.0, y: 100.0, z: 100.0},
            radius: 50.0,

            material: ModifiedPhongModel {
                emission:     &NoTexture { color: VEC3_ZERO },
                k_specular:   &NoTexture { color: Vec3 { x: 0.7, y: 0.7, z: 0.7 } },
                k_ambient:   &NoTexture { color: VEC3_ZERO },
                k_diffus:     &NoTexture { color: Vec3 { x: 0.81, y: 0.68, z: 0.21} },
                k_t:          &NoTexture { color: VEC3_ZERO },

                phong_exponent: 3.0,
                ior: 0.0,
            }
        },
        &Sphere {
            center: Vec3 {x: -100.0, y: -50.0, z: 80.0},
            radius: 0.0,
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: Vec3 { x: 0.7e4, y: 0.7e4, z: 0.7e4 } },
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
