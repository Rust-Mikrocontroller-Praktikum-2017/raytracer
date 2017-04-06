use vector::Vec3;
use intersection::Intersectable;
use intersection::Sphere;
use lcd::Color;

pub struct Scene {
    pub objects : &'static [&'static (Intersectable+Sync)]
}

pub static SCENE_SPHERE: Scene = Scene {
    objects: &[
        &Sphere { center: Vec3 {x: 0.0, y: 0.0, z: 0.0}, radius: 0.0, material: Color {red: 255, green: 0, blue: 0, alpha: 0}},
        &Sphere { center: Vec3 {x: 1.0, y: 1.0, z: 1.0}, radius: 1.0, material: Color {red: 0, green: 255, blue: 0, alpha: 0}},
    ]
};
