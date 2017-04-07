use vector::{Vec2, Vec3, VEC3_ZERO};
use math::{PI,tan};
use ray::Ray;

const UP: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 1.0};

pub trait Camera {
    fn gen_primary_ray(&self, x :f32, y :f32) -> Ray;
    fn get_position(&self) -> Vec3;
    fn get_target(&self) -> Vec3;
    fn get_film(&self) -> &Film;
}

pub struct OrthographicCamera {
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
}

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

pub struct Film {
    pub x_resolution: u32,
    pub y_resolution: u32,
    pub color: Vec3,
}

impl Camera for OrthographicCamera {
    fn gen_primary_ray(&self, x :f32, y :f32) -> Ray {
        let uv = make_uv(&self.film, self.t, self.r, self.b, self.l, x, y);
        let origin = self.u.mult(uv.u).add(&(self.v.mult(uv.v)));
        Ray::new(origin, self.w.mult(-1.0))
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

impl OrthographicCamera {
    pub fn new(pos :Vec3, target :Vec3, film :Film) -> OrthographicCamera {
        let mut cam = OrthographicCamera {
            pos: pos,
            target: target,
            film: film,

            l: 0, r: 0, t: 0, b: 0,
            u: VEC3_ZERO, v: VEC3_ZERO, w: VEC3_ZERO
        };

        let coord = make_camera_coord(&cam);
        cam.u = coord.0;
        cam.v = coord.1;
        cam.w = coord.2;

        let image_plane = make_image_plane(&cam);
        cam.l = image_plane.0;
        cam.r = image_plane.1;
        cam.t = image_plane.2;
        cam.b = image_plane.3;

        cam
    }
}

fn make_camera_coord(cam :&Camera) -> (Vec3,Vec3,Vec3) {
        let mut w = cam.get_position().sub(&cam.get_target());
        w.normalize();

        let u = UP.cross(&w);
        let v = w.cross(&u);

        (u,v,w)
}

fn make_image_plane(cam :&Camera) -> (i32, i32, i32, i32) {
        let t = (cam.get_film().y_resolution / 2) as i32;
        let b = -t;
        let r = (cam.get_film().x_resolution / 2) as i32;
        let l = -r;
        (t, b, r, l)
}

pub fn make_uv(film :&Film, t:i32,r:i32,b:i32,l:i32, x: f32, y: f32) -> Vec2 {
    Vec2 {
        u: l as f32 + ((r - l) as f32 * (x)) / (film.x_resolution as f32),
        v: t as f32 + ((b - t) as f32 * (y)) / (film.y_resolution as f32),
    }
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
        cam.l = image_plane.0;
        cam.r = image_plane.1;
        cam.t = image_plane.2;
        cam.b = image_plane.3;

        let fov_rad = (fov % 180) as f32 / 180.0 * PI;
        cam.fov = fov;
        cam.focal_length = cam.t as f32 / tan(fov_rad/2.0);
        
        cam
    }
}

impl Camera for PerspectiveCamera {

    fn gen_primary_ray(&self, x :f32, y :f32) -> Ray {
        let uv = make_uv(&self.film, self.t, self.r, self.b, self.l, x, y);
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
