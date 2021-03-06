use vector::Vec3;
use intersection::Intersectable;

pub trait Texture {
    fn get_texel(&self, u :f32, v:f32) -> Vec3;
}

pub trait TextureMapping {
    fn map_texture(&self, surface_pos :&Vec3, isectable: &Intersectable) -> Vec3;
}

pub struct MultTexture<'a> {
    pub factor: f32,
    pub texture: &'a Texture
}

impl<'a> Texture for MultTexture<'a> {
    fn get_texel(&self, u :f32, v:f32) -> Vec3 {
        self.texture.get_texel(u,v).mult(self.factor)
    }
}

pub enum CombineOp {
    ADD,
    MULT
}

pub struct CombineTexture<'a> {
    pub texture1: &'a Texture,
    pub texture2: &'a Texture
}

impl<'a> Texture for CombineTexture<'a> {
    fn get_texel(&self, u :f32, v:f32) -> Vec3 {
        self.texture1.get_texel(u,v).add(&self.texture2.get_texel(u,v))
    }
}
