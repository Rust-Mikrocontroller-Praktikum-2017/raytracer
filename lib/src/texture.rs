use vector::Vec3;

pub trait Texture {
    fn get_texel(&self, u :f32, v:f32) -> Vec3;
}

pub trait TextureMapping {
    fn map_texture(&self, surface_pos :&Vec3) -> Vec3;
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
