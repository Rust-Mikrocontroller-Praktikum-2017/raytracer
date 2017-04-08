use camera::{Film, Camera};
use ray::Ray;
use vector::{Vec3, VEC3_ZERO};
use math::{PI,tan};
use cameras::projective::{make_camera_coord, make_image_plane, make_uv};

pub struct PerspectiveCamera {
    // camera position in world coordinates
    pub pos: Vec3,
    pub target: Vec3,
    pub film: Film,

    // basis vectors of camera coordinate system
    // in world coordinates
    pub w: Vec3,
    pub v: Vec3,
    pub u: Vec3,

    // camera frame top, left, bottom, right.
    pub t :i32,
    pub l :i32,
    pub b :i32,
    pub r :i32,

    // distance between camera and image plane
    // (focal length)
    pub focal_length   :f32,

    // field of view in degree
    pub fov :u8
}

impl PerspectiveCamera {
    pub fn new(pos :Vec3, target :Vec3, film :Film) -> PerspectiveCamera {
        PerspectiveCamera::new_fov(pos, target, film, 90)
    }

    pub fn new_fov(pos :Vec3, target :Vec3, film :Film, fov: u8) -> PerspectiveCamera {
        let mut cam = PerspectiveCamera {
            pos: pos,
            target: target,
            film: film,
            fov: fov,

            focal_length: 0.0,
            l: 0, r: 0, t: 0, b: 0,
            u: VEC3_ZERO, v: VEC3_ZERO, w: VEC3_ZERO
        };

        let coord = make_camera_coord(&cam);
        cam.u = coord.0;
        cam.v = coord.1;
        cam.w = coord.2;

        let image_plane = make_image_plane(&cam);
        cam.t = image_plane.0;
        cam.r = image_plane.1;
        cam.b = image_plane.2;
        cam.l = image_plane.3;

        let fov_rad = (fov % 180) as f32 / 180.0 * PI;
        cam.fov = fov;
        cam.focal_length = cam.t as f32 / tan(fov_rad/2.0);

        cam
    }
}

impl Camera for PerspectiveCamera {

    fn gen_primary_ray(&self, x :f32, y :f32) -> Ray {
        let uv = make_uv(&self.film, (self.t,self.r,self.b,self.l), x, y);
        let mut dir = (self.u.mult(uv.u))
                .add(&(self.v.mult(uv.v)))
                .sub(&(self.w.mult(self.focal_length)));
        dir.normalize();
        Ray::new(self.pos, dir)
    }

    fn get_position(&self) -> Vec3 {
        self.pos
    }

    fn get_target(&self) -> Vec3 {
        self.target
    }

    fn get_film(&self) -> &Film {
        &self.film
    }
}
