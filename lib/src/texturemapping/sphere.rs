use texture::{Texture, TextureMapping};
use math::{arccos, arctan, arcsin, TWOPI};
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

pub struct Cylindrical<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for Cylindrical<'a> {
    fn map_texture(&self, surface_pos :&Vec3) -> Vec3 {
        let cylindrical = cartesian_to_cylindrical(surface_pos);
        let size = 2.0 * surface_pos.length();

        self.texture.get_texel(cylindrical.0 / TWOPI, cylindrical.1 / size)
    }
}

fn cartesian_to_cylindrical(v :&Vec3) -> (f32,f32) {
    (arcsin(v.y / v.length()), v.z)
}

pub struct MapXY<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for MapXY<'a> {
    fn map_texture(&self, surface_pos :&Vec3) -> Vec3 {
        let objsize = 2.0 * surface_pos.length();
        self.texture.get_texel(surface_pos.x / objsize, surface_pos.y / objsize)
    }
}

pub struct MapXZ<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for MapXZ<'a> {
    fn map_texture(&self, surface_pos :&Vec3) -> Vec3 {
        let objsize = surface_pos.length();
        self.texture.get_texel(surface_pos.x / objsize, surface_pos.z / objsize)
    }
}

pub struct MapYZ<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for MapYZ<'a> {
    fn map_texture(&self, surface_pos :&Vec3) -> Vec3 {
        let objsize = surface_pos.length();
        self.texture.get_texel(surface_pos.y / objsize, surface_pos.z / objsize)
    }
}
