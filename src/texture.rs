use vector::Vec3;

pub trait Texture {
    fn get_texel(&self, u :f32, v:f32) -> Vec3;
}

pub trait TextureMapping {
    fn map_texture(&self, surface_pos :&Vec3) -> Vec3;
}

struct A {
    f : fn(f32) -> f32
}
