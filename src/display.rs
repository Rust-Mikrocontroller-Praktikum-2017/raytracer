use vector::Vec3;

pub trait Display {
    fn set_pixel(&mut self, x :u16, y :u16, color: &Vec3);
    //fn accumulate_pixel(&self, x :u16, y :u16, color: &Vec3);
    fn reset(&mut self);
}
