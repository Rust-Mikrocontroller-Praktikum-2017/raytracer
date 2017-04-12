use scene::Scene;
use intersectables::sphere::Sphere;
use vector::{Vec3, VEC3_ZERO};
use reflectionmodel::ModifiedPhongModel;
use textures::color::NoTexture;
use textures::noise::EARTH_TEXTURE;
use texturemapping::sphere::MapXY;
use texture::MultTexture;

pub const SCENE_SPACE: Scene = Scene {
    objects: &[
        //&Sphere {
            //center: Vec3 {x: 00.0, y: 150.0, z: 50.0},
            //radius: 50.0,

            //material: ModifiedPhongModel {
                //emission:     &NoTexture { color: VEC3_ZERO },
                //k_specular:   &NoTexture { color: Vec3 { x: 0.0, y: 0.1, z: 0.1 } },
                //k_diffus:     &NoTexture { color: Vec3 { x: 0.70, y: 0.25, z: 0.25 } },
                //k_ambient:    &NoTexture { color: Vec3 { x: 0.20, y: 0.00, z: 0.05 } },
                //k_t:          &NoTexture { color: VEC3_ZERO },

                //phong_exponent: 1.0,
                //ior: 0.0,
            //}
        //},
        //&Sphere {
            //center: Vec3 {x: 0.0, y: 0.0, z: 50.0},
            //radius: 50.0,

            //material: ModifiedPhongModel {
                //emission:     &NoTexture { color: VEC3_ZERO },
                //k_specular:   &NoTexture { color: Vec3 { x: 0.3, y: 0.6, z: 0.6 } },
                //k_diffus:     &NoTexture { color: Vec3 { x: 0.70, y: 0.25, z: 0.25 } },
                //k_ambient:    &NoTexture { color: Vec3 { x: 0.20, y: 0.00, z: 0.05 } },
                //k_t:          &NoTexture { color: VEC3_ZERO },

                //phong_exponent: 3.0,
                //ior: 0.0,
            //}
        //},
        &Sphere {
            center: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            radius: 50.0,

            material: ModifiedPhongModel {
                emission:     &NoTexture { color: VEC3_ZERO },
                k_specular:   &NoTexture { color: Vec3 { x: 0.0, y: 0.0, z: 0.0 } },
                //k_specular:   &NoTexture { color: Vec3 { x: 0.5*(1.0/0.8), y: 0.8*(1.0/0.8), z: 0.8*(1.0/0.8) } },
                //k_diffus:     &Spherical { texture: &Tiles {} },
                k_diffus:     &MapXY { texture: &MultTexture { factor: 0.7, texture: &EARTH_TEXTURE}},
                k_ambient:    &MapXY { texture: &MultTexture { factor: 0.2, texture: &EARTH_TEXTURE}},
                //k_ambient:    &NoTexture { color: Vec3 { x: 0.20, y: 0.00, z: 0.05 } },
                k_t:          &NoTexture { color: VEC3_ZERO },

                phong_exponent: 3.0,
                ior: 0.0,
            }
        },
        //&Sphere {
            //center: Vec3 {x: -100.0, y: -50.0, z: 80.0},
            //radius: 0.0,
            //material: ModifiedPhongModel {
                //emission:       &NoTexture { color: Vec3 { x: 1.0e4, y: 1.0e4, z: 1.0e4 } },
                //k_specular:     &NoTexture { color: VEC3_ZERO },
                //k_diffus:       &NoTexture { color: VEC3_ZERO },
                //k_ambient:      &NoTexture { color: VEC3_ZERO },
                //k_t:            &NoTexture { color: VEC3_ZERO },

                //phong_exponent: 1.0,
                //ior: 0.0,
            //}
        //},
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
