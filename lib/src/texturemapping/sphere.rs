use texture::{Texture, TextureMapping};
use math::{arccos, arctan, arcsin, TWOPI, PI};
use vector::Vec3;
use intersection::Intersectable;

pub struct Spherical<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for Spherical<'a> {
    fn map_texture(&self, surface_pos :&Vec3, isectable: &Intersectable) -> Vec3 {
        let spherical = cartesian_to_spherical(surface_pos, isectable);

        self.texture.get_texel(spherical.0 / TWOPI, spherical.1 / PI)
    }
}

fn cartesian_to_spherical(v :&Vec3, isectable: &Intersectable) -> (f32,f32) {
    let center = &isectable.reduce_to_point();
    let v_to_center = Vec3::fromto(center, v);
    let len = isectable.maximum_expansion(center);
    (arccos(v_to_center.z / len), arctan(v_to_center.y/v_to_center.x))
}

pub struct Cylindrical<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for Cylindrical<'a> {
    fn map_texture(&self, surface_pos :&Vec3, isectable: &Intersectable) -> Vec3 {
        let cylindrical = cartesian_to_cylindrical(surface_pos, isectable);
        let size = 2.0 * surface_pos.length(); //???

        self.texture.get_texel(cylindrical.0 / TWOPI, cylindrical.1 / size)
    }
}

fn cartesian_to_cylindrical(v :&Vec3, isectable: &Intersectable) -> (f32,f32) {
    let center = &isectable.reduce_to_point();
    let v_to_center = Vec3::fromto(center, v);
    let len = isectable.maximum_expansion(center);
    (arcsin(v_to_center.y / len), v_to_center.z)
}

pub struct MapXY<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for MapXY<'a> {
    fn map_texture(&self, surface_pos :&Vec3, _isectable: &Intersectable) -> Vec3 {
        let objsize = 2.0 * surface_pos.length();
        self.texture.get_texel(surface_pos.x / objsize, surface_pos.y / objsize)
    }
}

pub struct MapXZ<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for MapXZ<'a> {
    fn map_texture(&self, surface_pos :&Vec3, _isectable: &Intersectable) -> Vec3 {
        let objsize = surface_pos.length();
        self.texture.get_texel(surface_pos.x / objsize, surface_pos.z / objsize)
    }
}

pub struct MapYZ<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for MapYZ<'a> {
    fn map_texture(&self, surface_pos :&Vec3, _isectable: &Intersectable) -> Vec3 {
        let objsize = surface_pos.length();
        self.texture.get_texel(surface_pos.y / objsize, surface_pos.z / objsize)
    }
}
