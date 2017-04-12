use scene::Scene;
use intersectables::sphere::Sphere;
use intersectables::triangle::Triangle;
use vector::{Vec3, VEC3_ZERO};
use reflectionmodel::ModifiedPhongModel;
use textures::color::{Tiles, NoTexture};
use textures::noise::{LaticeNoise, EARTH_TEXTURE};
use texturemapping::sphere::{MapXY, MapXZ, MapYZ, Cylindrical, Spherical};
use texture::MultTexture;
// use textures::noise::EARTH_TEXTURE;

pub const SCENE_SPHERE: Scene = Scene {
    objects: &[
        &Sphere {
            center: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            radius: 50.0,

            material: ModifiedPhongModel {
                emission:     &NoTexture { color: VEC3_ZERO },
                //k_specular:   &NoTexture { color: Vec3 { x: 0.0, y: 0.0, z: 0.0 } },
                k_specular:   &NoTexture { color: Vec3 { x: 0.9, y: 0.9, z: 0.9 } },
                //k_diffus:     &Cylindrical { texture: &Tiles {} },
                //k_diffus:     &Cylindrical { texture: &MultTexture { factor: 0.9, texture: &EARTH_TEXTURE}},
                //k_ambient:     &Cylindrical { texture: &MultTexture { factor: 0.4, texture: &EARTH_TEXTURE}},
                //k_diffus:   &NoTexture { color: VEC3_ZERO },
                k_ambient:   &NoTexture { color: VEC3_ZERO },
                //k_ambient:    &MapYZ { texture: &MultTexture { factor: 1.0, texture: &LaticeNoise {width: 5, height: 5}}},
                k_diffus:     &NoTexture { color: Vec3 { x: 0.50, y: 0.50, z: 0.50 } },
                //k_ambient:    &NoTexture { color: Vec3 { x: 0.20, y: 0.00, z: 0.05 } },
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
