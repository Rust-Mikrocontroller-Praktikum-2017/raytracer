use std::vec::Vec;
use intersection::Intersectable;
use intersection::Sphere;

pub struct Scene<'a> {
    pub objects : &'a [Intersectable]
}

static SCENE_SPHERE: &'static Scene = {
    objects: [
        Sphere { center: Vec3::new(0.0,0.0,0.0), radius: 1.0 }
    ]
}
