use intersection::Intersectable;

pub struct Scene {
    pub objects : &'static [&'static Intersectable]
}
