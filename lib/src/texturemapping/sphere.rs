use texture::{Texture, TextureMapping};
use math::{arccos, atan2, arctan, arcsin, TWOPI, PI};
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
        let center = &isectable.reduce_to_point();
        let offset = Vec3::fromto(center, surface_pos);
        let maxheight = 2.0 * isectable.maximum_expansion(center);

        let phi = atan2(offset.y, offset.x);

        self.texture.get_texel(phi / TWOPI, offset.z / maxheight)
    }
}

pub struct MapXY<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for MapXY<'a> {
    fn map_texture(&self, surface_pos :&Vec3, isectable: &Intersectable) -> Vec3 {
        let center = isectable.reduce_to_point();
        let objsize = isectable.maximum_expansion(&center) * 2.0;
        let offset = Vec3::fromto(&center, surface_pos);
        let mut s = offset.x / objsize;
        let s = (1.0 + s) * 0.5;
        let mut t = offset.y / objsize;
        let t = (1.0 + t) * 0.5;
        self.texture.get_texel(s, t)
    }
}

pub struct MapXZ<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for MapXZ<'a> {
    fn map_texture(&self, surface_pos :&Vec3, isectable: &Intersectable) -> Vec3 {
        let center = isectable.reduce_to_point();
        let objsize = isectable.maximum_expansion(&center) * 2.0;
        let offset = Vec3::fromto(&center, surface_pos);
        let mut s = offset.x / objsize;
        let s = (1.0 + s) * 0.5;
        let mut t = offset.z / objsize;
        let t = (1.0 + t) * 0.5;
        self.texture.get_texel(s, t)
    }
}

pub struct MapYZ<'a> {
    pub texture: &'a Texture
}

impl<'a> TextureMapping for MapYZ<'a> {
    fn map_texture(&self, surface_pos :&Vec3, isectable: &Intersectable) -> Vec3 {
        let center = isectable.reduce_to_point();
        let objsize = isectable.maximum_expansion(&center) * 2.0;
        let offset = Vec3::fromto(&center, surface_pos);
        let mut s = offset.y / objsize;
        let s = (1.0 + s) * 0.5;
        let mut t = offset.z / objsize;
        let t = (1.0 + t) * 0.5;
        self.texture.get_texel(s, t)
    }
}
