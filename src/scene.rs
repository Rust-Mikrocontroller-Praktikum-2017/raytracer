use vector::Vec3;
use intersection::Intersectable;
use intersection::Sphere;

pub struct Scene {
    pub objects : &'static [&'static Intersectable]
}

static SCENE_SPHERE: Scene = Scene {
    objects: &[
        &Sphere { center: Vec3::new(0.0,0.0,0.0), radius: 1.0 }
    ]
};
