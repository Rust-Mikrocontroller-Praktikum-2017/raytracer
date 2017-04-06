use vector::Vec3;
use intersection::Intersectable;
use intersection::Sphere;
use lcd::Color;

pub struct Scene {
    pub objects : &'static [&'static Intersectable]
}

static SCENE_SPHERE: Scene = Scene {
    objects: &[
        &Sphere { center: Vec3::new(0.0,0.0,0.0), radius: 1.0, material: Color::rgb(255,0,0) },
        &Sphere { center: Vec3::new(1.0,1.0,1.0), radius: 1.0, material: Color::rgb(0,0,255) },
    ]
};
