use vector::Vec3;
use ray::Ray;
//use scene::Scene;
//use display::Display;
//use render::render;

pub const UP: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 1.0};

pub trait Camera {
    fn gen_primary_ray(&self, x :f32, y :f32) -> Ray;
    fn get_position(&self) -> Vec3;
    fn get_target(&self) -> Vec3;
    fn get_film(&self) -> &Film;
    //fn take_picture(&self, scene :&Scene, display :&mut Display) {
        //render(display, self, scene);
    //}
}

pub struct Film {
    pub x_resolution: u32,
    pub y_resolution: u32,
    pub supersampling: u32,
    pub color: Vec3,
    pub iso: u32
}

impl Film {
    pub fn develop(&self, color :Vec3, _x: f32, _y: f32) -> Vec3 {
        color.mult(self.iso as f32 / 100.0)
    }
}