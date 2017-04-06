use vector::Vec3;
use math::{tan, PI};

pub struct Camera {

    // camera position in world coordinates
    pub pos: Vec3,

    // basis vectors of camera coordinate system
    // in world coordinates
    pub w: Vec3,
    pub v: Vec3,
    pub u: Vec3,

    // distance between camera and image plane
    // (focal length)
    pub focal_dist   :f64,

    pub frame_left   :i32,
    pub frame_right  :i32,
    pub frame_bottom :i32,
    pub frame_top    :i32,
}

const UP: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 1.0};

impl Camera {
    pub fn new(pos: &Vec3, target: &Vec3) -> Camera {
        Camera::new_focal(pos, target, 64.0)
    }
    pub fn new_focal(pos: &Vec3, target: &Vec3, focal_dist: f64) -> Camera {
        Camera::new_focal_fov(pos, target, focal_dist, 90)
    }
    pub fn new_focal_fov(pos: &Vec3, target: &Vec3, focal_dist: f64, fov: u8) -> Camera {
        let fov_rad = ((fov % 180) as f64) * PI / 180.0;
        let mut w = pos.sub(target);
        w.normalize();
        let u = UP.cross(&w);
        let v = w.cross(&u);
        let t = (tan(fov_rad/2.0) * focal_dist) as i32;
        let b = -t;
        let r = 480/272 * t;
        let l = -r;
        Camera {
            pos: *pos,
            w: w,
            u: u,
            v: v,
            focal_dist: focal_dist,
            frame_top: t,
            frame_bottom: b,
            frame_right: r,
            frame_left: l,
        }
    }
}
