use scene::Scene;
use intersectables::sphere::Sphere;
use intersectables::triangle::Triangle;
use vector::{Vec3, VEC3_ZERO};
use reflectionmodel::ModifiedPhongModel;
use textures::color::NoTexture;
// use textures::noise::EARTH_TEXTURE;

pub const SCENE_SPHERE: Scene = Scene {
    objects: &[
        &Sphere {
            center: Vec3 {x: 50.0, y: 50.0, z: 50.0},
            radius: 50.0,

            material: ModifiedPhongModel {
                emission:     &NoTexture { color: VEC3_ZERO },
                k_specular:   &NoTexture { color: VEC3_ZERO },
                k_diffus:     &NoTexture { color: Vec3 { x: 0.70, y: 0.00, z: 0.05 } },
                k_ambient:    &NoTexture { color: Vec3 { x: 0.20, y: 0.00, z: 0.20 } },
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
                k_specular:   &NoTexture { color: Vec3 { x: 0.7, y: 0.4, z: 0.4 } },
                k_diffus:     &NoTexture { color: Vec3 { x: 0.4, y: 0.0, z: 0.0 } },
                k_ambient:    &NoTexture { color: Vec3 { x: 0.1, y: 0.0, z: 0.0 } },
                k_t:          &NoTexture { color: VEC3_ZERO },

                phong_exponent: 4.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 { x: 240.0, y: -190.0, z: -40.0 },
            b: Vec3 { x: 240.0, y: 100.0, z: -40.0 },
            c: Vec3 { x: -240.0, y: -190.0, z: -40.0 },
            normal: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
            vec_ab: Vec3 { x: 0.0, y: 290.0, z: 0.0 },
            vec_ac: Vec3 { x: -480.0, y: 0.0, z: 0.0 },
            material: ModifiedPhongModel {
                emission:     &NoTexture { color: VEC3_ZERO },
                k_specular:   &NoTexture { color: VEC3_ZERO },
                k_diffus:     &NoTexture { color: Vec3 { x: 0.0, y: 0.7, z: 0.7 } },
                k_ambient:    &NoTexture { color: Vec3 { x: 0.0, y: 0.1, z: 0.1 } },
                k_t:          &NoTexture { color: VEC3_ZERO },

                phong_exponent: 4.0,
                ior: 0.0,
            }
        },
        &Sphere {
            center: Vec3 {x: -100.0, y: -100.0, z: 80.0},
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
