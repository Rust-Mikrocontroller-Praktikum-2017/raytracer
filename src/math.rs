use core::intrinsics;

pub const NAN: f64 = 0.0_f64/0.0_f64;

#[inline]
pub fn sqrt(x : f64) -> f64 {
    if x < 0.0 {
        NAN
    } else {
        unsafe { intrinsics::sqrtf64(x) }
    }
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
