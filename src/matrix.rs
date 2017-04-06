use vector::Vec3;
// Matrix class for homogeneneous coordinate computations

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub entries: [[f32; 4]; 4]
}

impl Mat4 {
    pub fn zero() -> Mat4 {
        Mat4 {
            entries: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
        }
    }

    pub fn identity() -> Mat4 {
        Mat4 {
            entries: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn translation(t :&Vec3) -> Mat4 {
        Mat4 {
            entries: [
                [1.0, 0.0, 0.0, t.x],
                [0.0, 1.0, 0.0, t.y],
                [0.0, 0.0, 1.0, t.z],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn scaling(s :&Vec3) -> Mat4 {
        Mat4 {
            entries: [
                [s.x, 0.0, 0.0, 0.0],
                [0.0, s.y, 0.0, 0.0],
                [0.0, 0.0, s.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }
}
