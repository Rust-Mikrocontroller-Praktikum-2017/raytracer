use vector::Vec3;
use ray::Ray;
use math::{sin, cos, HALFPI, min};
use cameras::projective::{make_camera_coord, make_image_plane};
//use scene::Scene;
//use display::Display;
//use render::render;

pub const UP: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 1.0};

pub enum Axis {
    X,
    Y,
    Z,
}

pub trait Camera {
    fn gen_primary_ray(&self, x :f32, y :f32) -> Ray;
    fn get_position(&self) -> Vec3;
    fn set_position(&mut self, pos: Vec3);
    fn get_target(&self) -> Vec3;
    fn get_film(&self) -> &Film;
    fn set_camera_coord(&mut self, coord: (Vec3,Vec3,Vec3));
    fn set_image_plane(&mut self, image_plane: (i32,i32,i32,i32));
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
pub trait CameraOperations {
    fn rotate(&mut self, axis: Axis, dist: f32);

}

impl<T : Camera> CameraOperations for T {
    fn rotate(&mut self, axis: Axis, dist: f32) {
        let pos = self.get_position();
        let mut new_pos = pos;
        match axis {
            Axis::X => {
                //unused
                let alpha = HALFPI;
                new_pos.y = cos(alpha) * pos.y - sin(alpha) * pos.z;
                new_pos.z = sin(alpha) * pos.y + cos(alpha) * pos.z;
            },
            Axis::Y => {
                let alpha = min(HALFPI, dist*1.25/272.0 * HALFPI);
                new_pos.x = cos(alpha) * pos.x + sin(alpha) * pos.z;
                new_pos.z = -sin(alpha) * pos.x + cos(alpha) * pos.z;
            },
            Axis::Z => {
                let alpha = min(HALFPI, dist*1.25/480.0 * HALFPI);
                new_pos.x = cos(alpha) * pos.x - sin(alpha) * pos.y;
                new_pos.y = sin(alpha) * pos.x + cos(alpha) * pos.y;
            },
        }

        self.set_position(new_pos);

        let coord = make_camera_coord(self);
        self.set_camera_coord(coord);

        let image_plane = make_image_plane(self);
        self.set_image_plane(image_plane);
    }
}
