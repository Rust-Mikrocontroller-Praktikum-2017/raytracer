use vector::Vec3;
use texture::{Texture, TextureMapping};

pub struct NoTexture {
    pub color: Vec3
}

impl Texture for NoTexture {
    fn get_texel(&self, _u :f32, _v: f32) -> Vec3 {
        self.color
    }
}

impl TextureMapping for NoTexture {
    fn map_texture(&self, _surface_pos :&Vec3) -> Vec3 {
        self.color
    }
}
