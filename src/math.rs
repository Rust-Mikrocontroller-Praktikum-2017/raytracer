use core::intrinsics;

pub const NAN: f32 = 0.0_f32/0.0_f32;
pub const PI: f32 = 3.14159265359;
pub const QTRPI: f32 = 0.25 * PI;
pub const HALFPI: f32 = 0.5 * PI;
pub const THREEHALFPI: f32 = 1.5 * PI;
pub const TWOPI: f32 = 2.0 * PI;
pub const FOUR_OVER_PI: f32 = 4.0 / PI;

#[inline]
pub fn sqrt(x : f32) -> f32 {
    if x < 0.0 {
        NAN
    } else {
        unsafe { intrinsics::sqrtf32(x) }
    }
}

#[inline]
pub fn modulo(x :f32, d :f32) -> f32 {
    ((x % d) + d) % d
}

pub fn sin(x :f32) -> f32 {
    cos(HALFPI - x)
}

// sin, cos and tan approximations adapted from
// http://www.ganssle.com/approx.htm

/// Accurate to about 3.2 decimal digits. Input argument is in radians.
pub fn cos(_x :f32) -> f32 {

    let mut x = _x % TWOPI;
    if x < 0.0 { x -= x; }

    let quadrant = (x / HALFPI) as u32;

    match quadrant {
        0 => return  cos_32s(x),
        1 => return -cos_32s(PI-x),
        2 => return -cos_32s(x-PI),
        3 => return  cos_32s(TWOPI-x),
        _ => unreachable!()
    }
}

const COS_1 :f32 = 0.99940307;
const COS_2 :f32 = -0.49558072;
const COS_3 :f32 = 0.03679168;

fn cos_32s(x :f32) -> f32 {
    let x2 = x*x;
    COS_1 + x2*(COS_2 + COS_3 * x2)
}

pub fn tan(_x :f32) -> f32 {
    let x = _x % TWOPI;
    let octant = (x / QTRPI) as u32;

    match octant {
        0 => return      tan_32s(x*FOUR_OVER_PI),
        1 => return  1.0/tan_32s((HALFPI-x)*FOUR_OVER_PI),
        2 => return -1.0/tan_32s((x-HALFPI)*FOUR_OVER_PI),
        3 => return     -tan_32s((PI-x)*FOUR_OVER_PI),
        4 => return      tan_32s((x-PI)*FOUR_OVER_PI),
        5 => return  1.0/tan_32s((THREEHALFPI-x)*FOUR_OVER_PI),
        6 => return -1.0/tan_32s((x-THREEHALFPI)*FOUR_OVER_PI),
        7 => return     -tan_32s((TWOPI-x)*FOUR_OVER_PI),
        _ => unreachable!()
    }
}

const TAN_1 :f32 = -3.6112171;
const TAN_2 :f32 = -4.6133253;

fn tan_32s(x: f32) -> f32 {
    let x2 = x*x;
    x*TAN_1/(TAN_2 + x2)
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
