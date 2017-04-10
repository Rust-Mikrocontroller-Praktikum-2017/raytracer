//#[cfg(test)]
//use std::intrinsics;
//#[cfg(not(test))]
use core::intrinsics;
use core::f32;

pub const EPS: f32 = 1.0e-3;
pub const NAN: f32 = f32::NAN;
pub const PI: f32 = f32::consts::PI;
pub const QTRPI: f32 = 0.25 * PI;
pub const HALFPI: f32 = 0.5 * PI;
pub const THREEHALFPI: f32 = 1.5 * PI;
pub const TWOPI: f32 = 2.0 * PI;
pub const FOUR_OVER_PI: f32 = 4.0 / PI;

pub fn min(a :f32, b :f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max(a :f32, b :f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn powi(x: f32, exp: u32) -> f32 {
    if exp == 0 {
        1.0
    } else {
        (0..exp).fold(1.0, |init,_| init*x)
    }
}

//#[inline]
//pub fn powf(x: f32, n: f32) -> f32 {
//    unsafe { intrinsics::powf32(x, n) }
//}

#[inline]
pub fn sqrt(x : f32) -> f32 {
    if x < 0.0 {
        NAN
    } else {
        unsafe { intrinsics::sqrtf32(x) }
    }
}

/// computes the remainder. Equals c % d in C or Rust.
/// Directly using % produces a linker error on the used hardware.
#[inline]
pub fn rem(c :f32, m :f32) -> f32 {
    c-(((c/m)+0.5) as u32 as f32 * m)
}

#[inline]
pub fn modulo(x :f32, d :f32) -> f32 {
    rem(rem(x,d) + d, d)
}

#[inline]
pub fn sin(x :f32) -> f32 {
    cos(HALFPI - x)
}

#[inline]
pub fn arcsin(x :f32) -> f32 {
    arctan(x / sqrt(1.0-2.0*x))
}

#[inline]
pub fn arccos(x :f32) -> f32 {
    HALFPI - arcsin(x)
}

pub fn arctan(x :f32) -> f32 {
    let mut _x = x;
    if x < -1.0 && x > 1.0 {
        _x = 1.0/x;
    }

    let mut sum = 0.0;
    for i in (0..10).filter(|i| i % 2 == 1) {
        let val = powi(_x, i) / (i as f32);
        if i % 4 == 1 {
            sum += val;
        } else {
            sum -= val;
        }
    }

    if x > -1.0 && x < 1.0 {
        sum
    } else {
        HALFPI - sum
    }
}

//const ATAN_1 :f32 = 1.6867629106;
//const ATAN_2 :f32 = 0.4378497304;
//const ATAN_3 :f32 = 1.6867633134;

//pub fn atan_66(x: f32) -> f32 {
    //let x2 = x*x;
    //(x*(ATAN_1+x2*ATAN_2)/ATAN_3+x2)
//}

// sin, cos and tan approximations adapted from
// http://www.ganssle.com/approx.htm

/// Accurate to about 3.2 decimal digits. Input argument is in radians.
pub fn cos(_x :f32) -> f32 {

    let mut x = rem(_x, TWOPI);
    if x < 0.0 { x -= x; }

    let quadrant = (x / HALFPI) as u32;

    match quadrant {
        0 =>  cos_32s(x),
        1 => -cos_32s(PI-x),
        2 => -cos_32s(x-PI),
        3 =>  cos_32s(TWOPI-x),
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
    let x = rem(_x, TWOPI);
    let octant = (x / QTRPI) as u32;

    match octant {
        0 =>      tan_32s(x*FOUR_OVER_PI),
        1 =>  1.0/tan_32s((HALFPI-x)*FOUR_OVER_PI),
        2 => -1.0/tan_32s((x-HALFPI)*FOUR_OVER_PI),
        3 =>     -tan_32s((PI-x)*FOUR_OVER_PI),
        4 =>      tan_32s((x-PI)*FOUR_OVER_PI),
        5 =>  1.0/tan_32s((THREEHALFPI-x)*FOUR_OVER_PI),
        6 => -1.0/tan_32s((x-THREEHALFPI)*FOUR_OVER_PI),
        7 =>     -tan_32s((TWOPI-x)*FOUR_OVER_PI),
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
