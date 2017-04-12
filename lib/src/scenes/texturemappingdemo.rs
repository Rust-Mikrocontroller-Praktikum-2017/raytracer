use scene::Scene;
use intersectables::sphere::Sphere;
use vector::{Vec3, VEC3_ZERO};
use reflectionmodel::ModifiedPhongModel;
use textures::color::{Tiles, NoTexture};
use texture::MultTexture;
use colors::{WHITE, RED};
use texturemapping::sphere::MapXY;

pub const SCENE_TEXMAP_DEMO: Scene = Scene {
    objects: &[
        &Sphere {
            center: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            radius: 50.0,

            material: ModifiedPhongModel {
                emission:     &NoTexture { color: VEC3_ZERO },
                k_specular:   &NoTexture { color: VEC3_ZERO },
                k_diffus:     &MapXY { texture: &MultTexture { factor: 0.8, texture: &Tiles { num_tiles_u: 7, num_tiles_v: 7, color_1: WHITE, color_2: RED } } },
                k_ambient:    &MapXY { texture: &MultTexture { factor: 0.25, texture: &Tiles { num_tiles_u: 7, num_tiles_v: 7, color_1: WHITE, color_2: RED } } },
                k_t:          &NoTexture { color: VEC3_ZERO },

                phong_exponent: 1.0,
                ior: 0.0,
            }
        },
        &Sphere {
            center: Vec3 {x: -95.0, y: -95.0, z: 90.0},
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
