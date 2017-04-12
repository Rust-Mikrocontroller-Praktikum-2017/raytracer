use scene::Scene;
use intersectables::sphere::Sphere;
use vector::{Vec3, VEC3_ZERO};
use reflectionmodel::ModifiedPhongModel;
use textures::color::{NoTexture};

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
        //&Sphere {
            //center: Vec3 {x: 400.0, y: -100.0, z: 130.0},
            //radius: 25.0,

            //material: ModifiedPhongModel {
                //emission:     &NoTexture { color: VEC3_ZERO },
                //k_specular:     &NoTexture { color: VEC3_ZERO },
                //k_specular:   &NoTexture { color: Vec3 { x: 1.0, y: 1.0, z: 1.0 } },
                //k_specular:   &NoTexture { color: Vec3 { x: 0.4, y: 0.4, z: 0.4 } },
                //k_diffus:     &Cylindrical { texture: &Tiles {} },
                //k_diffus:     &Cylindrical { texture: &MultTexture { factor: 0.9, texture: &EARTH_TEXTURE}},
                //k_ambient:     &Cylindrical { texture: &MultTexture { factor: 0.4, texture: &EARTH_TEXTURE}},
                //k_diffus:   &NoTexture { color: VEC3_ZERO },
                //k_ambient:   &NoTexture { color: VEC3_ZERO },
                //k_ambient:    &MapYZ { texture: &MultTexture { factor: 1.0, texture: &LaticeNoise {width: 5, height: 5}}},
                //k_diffus:     &NoTexture { color: VEC3_ZERO },
                //k_diffus:    &NoTexture { color: Vec3 { x: 0.10, y: 0.10, z: 0.10 } },
                //k_ambient:    &NoTexture { color: Vec3 { x: 0.02, y: 0.02, z: 0.02 } },
                //k_t:   &NoTexture { color: Vec3 { x: 0.9, y: 0.9, z: 0.9 } },

                //phong_exponent: 2.0,
                //ior: 1.5,
                //k_specular:   &NoTexture { color: Vec3 { x: 0.3, y: 0.3, z: 0.3 } },
                //k_specular:   &Cylindrical { texture: &Tiles{ num_tiles_u: 10, num_tiles_v: 10, color_1: Vec3 { x: 0.5, y: 0.5, z: 0.5 }, color_2: Vec3 { x: 0.2, y: 0.2, z: 0.2}}},
                //k_diffus:     &Cylindrical { texture: &Tiles {} },
                //k_diffus:     &Cylindrical { texture: &MultTexture { factor: 0.9, texture: &EARTH_TEXTURE}},
                //k_ambient:     &Cylindrical { texture: &MultTexture { factor: 0.4, texture: &EARTH_TEXTURE}},
                //k_diffus:   &NoTexture { color: VEC3_ZERO },
                //k_ambient:   &NoTexture { color: VEC3_ZERO },
                //k_ambient:    &MapYZ { texture: &MultTexture { factor: 1.0, texture: &LaticeNoise {width: 5, height: 5}}},
                //k_diffus:     &Cylindrical { texture: &Tiles{ num_tiles_u: 10, num_tiles_v: 10, color_1: Vec3 { x: 0.9, y: 0.9, z: 0.0 }, color_2: Vec3 { x: 0.0, y: 0.9, z: 0.9}}},
                //k_ambient:     &Cylindrical { texture: &Tiles{ num_tiles_u: 10, num_tiles_v: 10, color_1: Vec3 { x: 0.3, y: 0.3, z: 0.0 }, color_2: Vec3 { x: 0.0, y: 0.3, z: 0.3}}},
                ////k_ambient:    &NoTexture { color: Vec3 { x: 0.10, y: 0.00, z: 0.10 } },
                //k_t:          &NoTexture { color: VEC3_ZERO },

                //phong_exponent: 1.0,
                //ior: 0.0,
            //}
        //},
        &Sphere {
            center: Vec3 {x: 200.0, y: 100.0, z: 100.0},
            radius: 50.0,

            material: ModifiedPhongModel {
                emission:     &NoTexture { color: VEC3_ZERO },
                //k_specular:   &NoTexture { color: Vec3 { x: 0.0, y: 0.0, z: 0.0 } },
                k_specular:   &NoTexture { color: Vec3 { x: 0.7, y: 0.7, z: 0.7 } },
                //k_diffus:     &Cylindrical { texture: &Tiles {} },
                //k_diffus:     &Cylindrical { texture: &MultTexture { factor: 0.9, texture: &EARTH_TEXTURE}},
                //k_ambient:     &Cylindrical { texture: &MultTexture { factor: 0.4, texture: &EARTH_TEXTURE}},
                //k_diffus:   &NoTexture { color: VEC3_ZERO },
                k_ambient:   &NoTexture { color: VEC3_ZERO },
                //k_ambient:    &MapYZ { texture: &MultTexture { factor: 1.0, texture: &LaticeNoise {width: 5, height: 5}}},
                k_diffus:     &NoTexture { color: Vec3 { x: 0.81, y: 0.68, z: 0.21} },
                //k_ambient:    &NoTexture { color: Vec3 { x: 0.20, y: 0.00, z: 0.05 } },
                k_t:          &NoTexture { color: VEC3_ZERO },

                phong_exponent: 3.0,
                ior: 0.0,
                //k_specular:   &NoTexture { color: Vec3 { x: 0.3, y: 0.3, z: 0.3 } },
                //k_specular:   &Cylindrical { texture: &Tiles{ num_tiles_u: 10, num_tiles_v: 10, color_1: Vec3 { x: 0.5, y: 0.5, z: 0.5 }, color_2: Vec3 { x: 0.2, y: 0.2, z: 0.2}}},
                //k_diffus:     &Cylindrical { texture: &Tiles {} },
                //k_diffus:     &Cylindrical { texture: &MultTexture { factor: 0.9, texture: &EARTH_TEXTURE}},
                //k_ambient:     &Cylindrical { texture: &MultTexture { factor: 0.4, texture: &EARTH_TEXTURE}},
                //k_diffus:   &NoTexture { color: VEC3_ZERO },
                //k_ambient:   &NoTexture { color: VEC3_ZERO },
                //k_ambient:    &MapYZ { texture: &MultTexture { factor: 1.0, texture: &LaticeNoise {width: 5, height: 5}}},
                //k_diffus:     &Cylindrical { texture: &Tiles{ num_tiles_u: 10, num_tiles_v: 10, color_1: Vec3 { x: 0.9, y: 0.9, z: 0.0 }, color_2: Vec3 { x: 0.0, y: 0.9, z: 0.9}}},
                //k_ambient:     &Cylindrical { texture: &Tiles{ num_tiles_u: 10, num_tiles_v: 10, color_1: Vec3 { x: 0.3, y: 0.3, z: 0.0 }, color_2: Vec3 { x: 0.0, y: 0.3, z: 0.3}}},
                ////k_ambient:    &NoTexture { color: Vec3 { x: 0.10, y: 0.00, z: 0.10 } },
                //k_t:          &NoTexture { color: VEC3_ZERO },

                //phong_exponent: 1.0,
                //ior: 0.0,
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
