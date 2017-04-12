use vector::Vec3;
use colors::{DEEP_BLUE, BLUE, CYAN, BROWN_ORANGE, GRASS_GREEN, WHITE, DARK_BLUE, BLACK};

pub trait ColorMapping {
    fn color_map(&self, val :f32) -> Vec3;
}

pub struct EarthTones {}

impl ColorMapping for EarthTones {
    fn color_map(&self, val :f32) -> Vec3 {
        if val < 0.4 {
            Vec3::lerp(&DEEP_BLUE, &BLUE, val / 0.4)
        } else if val < 0.5 {
            Vec3::lerp(&BLUE, &CYAN, (val - 0.4) / 0.1)
        } else if val < 0.6 {
            BROWN_ORANGE
        } else if val < 1.0 {
            Vec3::lerp(&GRASS_GREEN, &WHITE, (val - 0.6) / 0.4)
        } else {
            WHITE
        }
    }
}

pub struct SpaceAndStars {}

impl ColorMapping for SpaceAndStars {
    fn color_map(&self, val :f32) -> Vec3 {
        if val < 0.5 {
            Vec3::lerp(&BLACK, &DARK_BLUE, val / 0.4)
        } else if val < 0.51 {
            BLUE
        } else if val < 0.99 {
            Vec3::lerp(&BLACK, &DARK_BLUE, val / 0.4)
        } else if val < 1.01 {
            Vec3::lerp(&Vec3::new(0.0,1.0,0.0), &WHITE, (val - 0.6) / 0.4)
        } else {
            BLACK
        }
    }
}
