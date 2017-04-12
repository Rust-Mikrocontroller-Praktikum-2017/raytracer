use scene::Scene;
use intersectables::sphere::Sphere;
use intersectables::triangle::Triangle;
use vector::{Vec3, VEC3_ZERO};
use reflectionmodel::ModifiedPhongModel;
use textures::color::{NoTexture};
use textures::file::FileTexture;
use texture::MultTexture;
use texturemapping::sphere::{MapXY};

pub const SCENE_PYRAMID: Scene = Scene {
    objects: &[
        &Triangle {
            a: Vec3 {x: 0.0, y: 0.0, z: 70.0},
            b: Vec3 {x: -50.0, y: 50.0, z: 0.0},
            c: Vec3 {x: -50.0, y: -50.0, z: 0.0},
            vec_ab: Vec3 {x: -50.0, y: 50.0, z: -70.0},
            vec_ac: Vec3 {x: -50.0, y: -50.0, z: -70.0},
            normal: Vec3{x: -0.813733459, y: 0.0, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &NoTexture { color: Vec3 { x: 0.7, y: 0.7, z: 0.0 } },
                k_ambient:      &NoTexture { color: Vec3 { x: 0.7, y: 0.7, z: 0.0 } },
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 0.0, y: 0.0, z: 70.0},
            b: Vec3 {x: 50.0, y: 50.0, z: 0.0},
            c: Vec3{x: -50.0, y: 50.0, z: 0.0},
            vec_ab: Vec3 {x: 50.0, y: 50.0, z: -70.0},
            vec_ac: Vec3 {x: -50.0, y: 50.0, z: -70.0},
            normal: Vec3 {x:0.0, y: 0.813733459, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &NoTexture { color: Vec3 { x: 0.7, y: 0.7, z: 0.0 } },
                k_ambient:      &NoTexture { color: Vec3 { x: 0.7, y: 0.7, z: 0.0 } },
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 0.0, y: 0.0, z: 70.0},
            b: Vec3 {x: 50.0, y: -50.0, z: 0.0},
            c: Vec3 {x: 50.0, y: 50.0, z: 0.0},
            vec_ab: Vec3 {x: 50.0, y: -50.0, z: -70.0},
            vec_ac: Vec3 {x: 50.0, y: 50.0, z: -70.0},
            normal: Vec3 {x:0.813733459, y: 0.0, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &NoTexture { color: Vec3 { x: 0.7, y: 0.7, z: 0.0 } },
                k_ambient:      &NoTexture { color: Vec3 { x: 0.7, y: 0.7, z: 0.0 } },
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 0.0, y: 0.0, z: 70.0},
            b: Vec3 {x: -50.0, y: -50.0, z: 0.0},
            c: Vec3 {x: 50.0, y: -50.0, z: 0.0},
            vec_ab: Vec3 {x: -50.0, y: -50.0, z: -70.0},
            vec_ac: Vec3 {x: 50.0, y: -50.0, z: -70.0},
            normal: Vec3{x: 0.0, y: -0.813733459, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &NoTexture { color: Vec3 { x: 0.7, y: 0.7, z: 0.0 } },
                k_ambient:      &NoTexture { color: Vec3 { x: 0.7, y: 0.7, z: 0.0 } },
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 335.2935, y: 191.404005, z: 0.0},
            b: Vec3 {x: -23.57535, y: 365.4375, z: 0.0},
            c: Vec3 {x: -335.2935, y: -147.23385, z: 0.0},
            vec_ab: Vec3 {x: -358.86885, y: 174.033495, z: 0.0},
            vec_ac: Vec3 {x: -670.587, y: -338.637855, z: 0.0},
            normal: Vec3{x: 0.0, y: 0.0, z: 1.0},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &MultTexture { factor: 0.7, texture: &FileTexture { width: 150, height: 150, rgbdata: include_bytes!("../../../textures/sand.rgb")}}},
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &FileTexture { width: 150, height: 150, rgbdata: include_bytes!("../../../textures/sand.rgb")}}},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 335.2935, y: 191.404005, z: 0.0},
            b: Vec3 {x: -335.2935, y: -147.23385, z: 0.0},
            c: Vec3 {x: 23.57535, y: -365.4375, z: 0.0},
            vec_ab: Vec3 {x: -670.587, y: -338.637855, z: 0.0},
            vec_ac: Vec3 {x: -311.71815, y: -556.841505, z: 0.0},
            normal: Vec3{x: 0.0, y: 0.0, z: 1.0},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &MultTexture { factor: 0.7, texture: &FileTexture { width: 150, height: 150, rgbdata: include_bytes!("../../../textures/sand.rgb")}}},
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &FileTexture { width: 150, height: 150, rgbdata: include_bytes!("../../../textures/sand.rgb")}}},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Sphere {
            center: Vec3 {x: 60.0, y: 100.0, z: 130.0},
            radius: 0.0,
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: Vec3 { x: 2.0e4, y: 2.0e4, z: 2.0e4 } },
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
