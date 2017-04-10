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
}

impl PerspectiveCamera {
    pub fn new(pos :Vec3, target :Vec3, film :Film) -> PerspectiveCamera {
        let mut cam = PerspectiveCamera {
            pos: pos,
            target: target,
            film: film,

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

        cam.set_field_of_view(90);

        cam
    }

    pub fn set_field_of_view(&mut self, fov :u8) -> &mut Self {
        let fov_rad = (fov % 180) as f32 / 180.0 * PI;
        let focal_length = self.t as f32 / tan(fov_rad/2.0);
        self.set_focal_length(focal_length);
        self
    }

    pub fn set_focal_length(&mut self, new_fl :f32) -> &mut Self {
        self.pos.inplace_add(&self.w.mult(-self.focal_length));
        self.focal_length = new_fl;
        self.pos.inplace_add(&self.w.mult(new_fl));

        self
    }
}

impl Camera for PerspectiveCamera {

    fn gen_primary_ray(&self, x :f32, y :f32) -> Ray {
        let uv = make_uv(&self.film, (self.t,self.r,self.b,self.l), x+0.5, y+0.5);
        let mut dir = (self.u.mult(uv.u))
                .add(&(self.v.mult(uv.v)))
                .sub(&(self.w.mult(self.focal_length)));
        dir.normalize();
        Ray::new(self.pos, dir)
    }

    fn get_position(&self) -> Vec3 {
        self.pos
    }

    fn set_position(&mut self, pos: Vec3) {
        self.pos = pos;
    }

    fn get_target(&self) -> Vec3 {
        self.target
    }

    fn get_film(&self) -> &Film {
        &self.film
    }

    fn set_image_plane(&mut self, image_plane: (i32,i32,i32,i32)) {
        self.t = image_plane.0;
        self.r = image_plane.1;
        self.b = image_plane.2;
        self.l = image_plane.3;
    }

    fn set_camera_coord(&mut self, coord: (Vec3,Vec3,Vec3)) {
        self.u = coord.0;
        self.v = coord.1;
        self.w = coord.2;
    }
}
