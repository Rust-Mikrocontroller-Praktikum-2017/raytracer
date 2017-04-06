use vector::Vec3;

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
    pub d            :f64,

    pub frame_left   :i32,
    pub frame_right  :i32,
    pub frame_bottom :i32,
    pub frame_top    :i32,
}
