use scene::Scene;
use intersectables::sphere::Sphere;
use intersectables::triangle::Triangle;
use vector::{Vec3, VEC3_ZERO};
use reflectionmodel::ModifiedPhongModel;
use textures::color::{NoTexture, Tiles};
use textures::noise::{Noise, LaticeNoise};
use texture::{MultTexture, CombineTexture};
use texturemapping::sphere::{MapXY};
use colors::{DARK_SAND, LIGHT_SAND};

pub const PYRAMID_STONE_TEXTURE: CombineTexture = CombineTexture {
    texture1: &Tiles { num_tiles_u: 100, num_tiles_v: 100, color_1: LIGHT_SAND, color_2: DARK_SAND },
    texture2: &MultTexture { factor: 0.4, texture: &Noise { } }
};

pub const SAND_TEXTURE: CombineTexture = CombineTexture {
    texture1: &NoTexture { color: LIGHT_SAND },
    texture2: //&CombineTexture {
        //texture1: &MultTexture { factor: 0.2, texture: &Noise { } },
        /*texture2:*/ &MultTexture { factor: 0.3, texture: &LaticeNoise {
            width: 800,
            height: 800,
            seed: 1
        }//}
    }
};

pub const SCENE_PYRAMID: Scene = Scene {
    objects: &[
        &Triangle {
            a: Vec3 {x: -60.0, y: 0.0, z: 70.0},
            b: Vec3 {x: -110.0, y: 50.0, z: 0.0},
            c: Vec3 {x: -110.0, y: -50.0, z: 0.0},
            vec_ab: Vec3 {x: -50.0, y: 50.0, z: -70.0},
            vec_ac: Vec3 {x: -50.0, y: -50.0, z: -70.0},
            normal: Vec3{x: -0.813733459, y: 0.0, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: -60.0, y: 0.0, z: 70.0},
            b: Vec3 {x: -10.0, y: 50.0, z: 0.0},
            c: Vec3{x: -110.0, y: 50.0, z: 0.0},
            vec_ab: Vec3 {x: 50.0, y: 50.0, z: -70.0},
            vec_ac: Vec3 {x: -50.0, y: 50.0, z: -70.0},
            normal: Vec3 {x:0.0, y: 0.813733459, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: -60.0, y: 0.0, z: 70.0},
            b: Vec3 {x: -10.0, y: -50.0, z: 0.0},
            c: Vec3 {x: -10.0, y: 50.0, z: 0.0},
            vec_ab: Vec3 {x: 50.0, y: -50.0, z: -70.0},
            vec_ac: Vec3 {x: 50.0, y: 50.0, z: -70.0},
            normal: Vec3 {x:0.813733459, y: 0.0, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: -60.0, y: 0.0, z: 70.0},
            b: Vec3 {x: -110.0, y: -50.0, z: 0.0},
            c: Vec3 {x: -10.0, y: -50.0, z: 0.0},
            vec_ab: Vec3 {x: -50.0, y: -50.0, z: -70.0},
            vec_ac: Vec3 {x: 50.0, y: -50.0, z: -70.0},
            normal: Vec3{x: 0.0, y: -0.813733459, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 70.0, y: 55.0, z: 70.0},
            b: Vec3 {x: 20.0, y: 105.0, z: 0.0},
            c: Vec3 {x: 20.0, y: 5.0, z: 0.0},
            vec_ab: Vec3 {x: -50.0, y: 50.0, z: -70.0},
            vec_ac: Vec3 {x: -50.0, y: -50.0, z: -70.0},
            normal: Vec3{x: -0.813733459, y: 0.0, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 70.0, y: 55.0, z: 70.0},
            b: Vec3 {x: 120.0, y: 105.0, z: 0.0},
            c: Vec3{x: 20.0, y: 105.0, z: 0.0},
            vec_ab: Vec3 {x: 50.0, y: 50.0, z: -70.0},
            vec_ac: Vec3 {x: -50.0, y: 50.0, z: -70.0},
            normal: Vec3 {x:0.0, y: 0.813733459, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 70.0, y: 55.0, z: 70.0},
            b: Vec3 {x: 120.0, y: 5.0, z: 0.0},
            c: Vec3 {x: 120.0, y: 105.0, z: 0.0},
            vec_ab: Vec3 {x: 50.0, y: -50.0, z: -70.0},
            vec_ac: Vec3 {x: 50.0, y: 50.0, z: -70.0},
            normal: Vec3 {x:0.813733459, y: 0.0, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 70.0, y: 55.0, z: 70.0},
            b: Vec3 {x: 20.0, y: 5.0, z: 0.0},
            c: Vec3 {x: 120.0, y: 5.0, z: 0.0},
            vec_ab: Vec3 {x: -50.0, y: -50.0, z: -70.0},
            vec_ac: Vec3 {x: 50.0, y: -50.0, z: -70.0},
            normal: Vec3{x: 0.0, y: -0.813733459, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 80.0, y: -100.0, z: 70.0},
            b: Vec3 {x: 30.0, y: -50.0, z: 0.0},
            c: Vec3 {x: 30.0, y: -150.0, z: 0.0},
            vec_ab: Vec3 {x: -50.0, y: 50.0, z: -70.0},
            vec_ac: Vec3 {x: -50.0, y: -50.0, z: -70.0},
            normal: Vec3{x: -0.813733459, y: 0.0, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 80.0, y: -100.0, z: 70.0},
            b: Vec3 {x: 130.0, y: -50.0, z: 0.0},
            c: Vec3{x: 30.0, y: -50.0, z: 0.0},
            vec_ab: Vec3 {x: 50.0, y: 50.0, z: -70.0},
            vec_ac: Vec3 {x: -50.0, y: 50.0, z: -70.0},
            normal: Vec3 {x:0.0, y: 0.813733459, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 80.0, y: -100.0, z: 70.0},
            b: Vec3 {x: 130.0, y: -150.0, z: 0.0},
            c: Vec3 {x: 130.0, y: -50.0, z: 0.0},
            vec_ab: Vec3 {x: 50.0, y: -50.0, z: -70.0},
            vec_ac: Vec3 {x: 50.0, y: 50.0, z: -70.0},
            normal: Vec3 {x:0.813733459, y: 0.0, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 80.0, y: -100.0, z: 70.0},
            b: Vec3 {x: 30.0, y: -150.0, z: 0.0},
            c: Vec3 {x: 130.0, y: -150.0, z: 0.0},
            vec_ab: Vec3 {x: -50.0, y: -50.0, z: -70.0},
            vec_ac: Vec3 {x: 50.0, y: -50.0, z: -70.0},
            normal: Vec3{x: 0.0, y: -0.813733459, z: 0.58123821},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &PYRAMID_STONE_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.2, texture: &PYRAMID_STONE_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 335.2935, y: 291.404005, z: 0.0},
            b: Vec3 {x: -23.57535, y: 465.4375, z: 0.0},
            c: Vec3 {x: -335.2935, y: -247.23385, z: 0.0},
            vec_ab: Vec3 {x: -358.86885, y: 174.033495, z: 0.0},
            vec_ac: Vec3 {x: -670.587, y: -538.637855, z: 0.0},
            normal: Vec3{x: 0.0, y: 0.0, z: 1.0},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &SAND_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.3, texture: &SAND_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Triangle {
            a: Vec3 {x: 335.2935, y: 291.404005, z: 0.0},
            b: Vec3 {x: -335.2935, y: -247.23385, z: 0.0},
            c: Vec3 {x: 23.57535, y: -465.4375, z: 0.0},
            vec_ab: Vec3 {x: -670.587, y: -538.637855, z: 0.0},
            vec_ac: Vec3 {x: -311.71815, y: -756.841505, z: 0.0},
            normal: Vec3{x: 0.0, y: 0.0, z: 1.0},
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: VEC3_ZERO },
                k_specular:     &NoTexture { color: VEC3_ZERO },
                k_diffus:       &MapXY { texture: &SAND_TEXTURE },
                k_ambient:      &MapXY { texture: &MultTexture { factor: 0.3, texture: &SAND_TEXTURE }},
                k_t:            &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Sphere {
            center: Vec3 {x: 700.0, y: 700.0, z: 750.0},
            radius: 0.0,
            material: ModifiedPhongModel {
                emission:       &NoTexture { color: Vec3 { x: 1.0e6, y: 1.0e6, z: 1.0e6 } },
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
