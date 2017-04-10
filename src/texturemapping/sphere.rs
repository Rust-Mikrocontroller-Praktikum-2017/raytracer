use texture::{Texture, TextureMapping};
use math::{arccos, arctan, TWOPI};
use vector::Vec3;

pub struct Spherical<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for Spherical<'a> {
    fn map_texture(&self, surface_pos :&Vec3) -> Vec3 {
        let spherical = cartesian_to_spherical(surface_pos);

        self.texture.get_texel(spherical.0 / TWOPI, spherical.1)
    }
}

fn cartesian_to_spherical(v :&Vec3) -> (f32,f32) {
    (arccos(v.z / v.length()), arctan(v.y/v.x))
}
