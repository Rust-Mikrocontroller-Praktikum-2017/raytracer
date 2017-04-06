use core::intrinsics;

pub const NAN: f32 = 0.0_f32/0.0_f32;
pub const PI: f32 = 3.14159265359;

#[inline]
pub fn sqrt(x : f32) -> f32 {
    if x < 0.0 {
        NAN
    } else {
        unsafe { intrinsics::sqrtf32(x) }
    }
}

#[inline]
pub fn tan(x: f32) -> f32 {
    x + x*x*x/3.0 + 2.0*x*x*x*x*x/15.0
}

//#[inline]
//pub fn wrapping_add(self, rhs: Self) -> Self {
    //unsafe {
        //intrinsics::overflowing_add(self, rhs)
    //}
//}

//#[inline]
//pub fn wrapping_sub(self, rhs: Self) -> Self {
    //unsafe {
        //intrinsics::overflowing_sub(self, rhs)
    //}
//}

//#[inline]
//pub fn wrapping_mul(self, rhs: Self) -> Self {
    //unsafe {
        //intrinsics::overflowing_mul(self, rhs)
    //}
//}

//#[inline(always)]
//pub fn wrapping_div(self, rhs: Self) -> Self {
    //self.overflowing_div(rhs).0
//}
