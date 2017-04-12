use vector::Vec3;
use texture::{Texture, TextureMapping};
use colors::{RED, WHITE};
use intersection::Intersectable;

pub struct NoTexture {
    pub color: Vec3
}

impl Texture for NoTexture {
    fn get_texel(&self, _u :f32, _v: f32) -> Vec3 {
        self.color
    }
}

impl TextureMapping for NoTexture {
    fn map_texture(&self, _surface_pos :&Vec3, _isectable: &Intersectable) -> Vec3 {
        self.color
    }
}

pub struct Tiles {
    pub num_tiles_u: u32,
    pub num_tiles_v: u32
}

impl Texture for Tiles {
    fn get_texel(&self, u :f32, v: f32) -> Vec3 {
        let tile_index_u = (u * self.num_tiles_u as f32) as u32;
        let tile_index_v = (v * self.num_tiles_v as f32) as u32;

        let even_u = (tile_index_u % 2) == 0;
        let even_v = (tile_index_v % 2) == 0;

        if even_u  {
            if even_v {
                RED
            } else {
                WHITE
            }
        } else {
            if even_v {
                WHITE
            } else {
                RED
            }
        }
    }
}
