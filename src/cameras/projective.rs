use camera::{Camera, Film, UP};
use vector::{Vec2, Vec3};

pub fn make_camera_coord(cam :&Camera) -> (Vec3,Vec3,Vec3) {
        let mut w = cam.get_position().sub(&cam.get_target());
        w.normalize();

        let u = UP.cross(&w);
        let v = w.cross(&u);

        (u,v,w)
}

pub fn make_image_plane(cam :&Camera) -> (i32, i32, i32, i32) {
        let t = (cam.get_film().y_resolution / 2) as i32;
        let b = -t;
        let r = (cam.get_film().x_resolution / 2) as i32;
        let l = -r;
        (t, r, b, l)
}

pub fn make_uv(film :&Film, (top,right,bottom,left): (i32,i32,i32,i32), x: f32, y: f32) -> Vec2 {
    Vec2 {
        u: left as f32 + ((right - left) as f32 * (x)) / (film.x_resolution as f32),
        v: top as f32 + ((bottom - top) as f32 * (y)) / (film.y_resolution as f32),
    }
}
