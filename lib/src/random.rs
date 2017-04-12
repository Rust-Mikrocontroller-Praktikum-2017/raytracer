// src/mt19937.rs
//
// Copyright (c) 2015,2017 rust-mersenne-twister developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

// Modifications from original:
// - removed dependency "rand"
// - removed u64 support
// - Stripped ordering, hashing and other traits
// - stripped tests

use core::num::Wrapping;

const N: usize = 624;
const M: usize = 397;
const MATRIX_A: Wrapping<u32> = Wrapping(0x9908b0df);
const UPPER_MASK: Wrapping<u32> = Wrapping(0x80000000);
const LOWER_MASK: Wrapping<u32> = Wrapping(0x7fffffff);

/// The 32-bit flavor of the Mersenne Twister pseudorandom number
/// generator.
//#[derive(Copy)]
pub struct MT19937 {
    idx: usize,
    state: [Wrapping<u32>; N],
}

const UNINITIALIZED: MT19937 = MT19937 {
    idx: 0,
    state: [Wrapping(0); N]
};

impl MT19937 {
    #[inline]
    fn from_seed(seed: u32) -> MT19937 {
        let mut mt = UNINITIALIZED;
        mt.reseed(seed);
        mt
    }

    fn reseed(&mut self, seed: u32) {
        self.idx = N;
        self.state[0] = Wrapping(seed);
        for i in 1..N {
            self.state[i] = Wrapping(1812433253) * (self.state[i-1] ^ (self.state[i-1]>>30)) + Wrapping(i as u32);
        }
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        // Failing this check indicates that, somehow, the structure
        // was not initialized.
        debug_assert!(self.idx != 0);
        if self.idx >= N {
            self.fill_next_state();
        }
        let Wrapping(x) = self.state[self.idx];
        self.idx += 1;
        temper(x)
    }

    /// Create a new Mersenne Twister random number generator using
    /// the default fixed seed.
    #[inline]
    pub fn new_unseeded() -> MT19937 {
        MT19937::from_seed(5489u32)
    }

    fn fill_next_state(&mut self) {
        static MAG01: [Wrapping<u32>; 2] = [Wrapping(0), MATRIX_A];
        for i in 0 .. N-M {
            let x = (self.state[i]&UPPER_MASK) | (self.state[i+1]&LOWER_MASK);
            self.state[i] = self.state[i+M] ^ (x>>1) ^ MAG01[(x.0&1) as usize];
        }
        for i in N-M .. N-1 {
            let x = (self.state[i]&UPPER_MASK) | (self.state[i+1]&LOWER_MASK);
            self.state[i] = self.state[i+M-N] ^ (x>>1) ^ MAG01[(x.0&1) as usize];
        }
        let x = (self.state[N-1]&UPPER_MASK) | (self.state[0]&LOWER_MASK);
        self.state[N-1] = self.state[M-1] ^ (x>>1) ^ MAG01[(x.0&1) as usize];
        self.idx = 0;
    }

    /// Recover the internal state of a Mersenne Twister instance
    /// from 624 consecutive outputs of the algorithm.
    ///
    /// The returned `MT19937` is guaranteed to identically reproduce
    /// subsequent outputs of the RNG that was sampled.
    ///
    /// Panics if the length of the slice is not exactly 624.
    pub fn recover(samples: &[u32]) -> MT19937 {
        assert!(samples.len() == N);
        let mut mt = UNINITIALIZED;
        for (in_, out) in Iterator::zip(samples.iter(), mt.state.iter_mut()) {
            *out = Wrapping(untemper(*in_));
        }
        mt.idx = N;
        mt
    }
}

#[inline]
fn temper(mut x: u32) -> u32 {
    x ^=  x>>11;
    x ^= (x<< 7) & 0x9d2c5680;
    x ^= (x<<15) & 0xefc60000;
    x ^=  x>>18;
    x
}

#[inline]
fn untemper(mut x: u32) -> u32 {
    // reverse "x ^=  x>>18;"
    x ^= x>>18;

    // reverse "x ^= (x<<15) & 0xefc60000;"
    x ^= (x<<15) & 0x2fc60000;
    x ^= (x<<15) & 0xc0000000;

    // reverse "x ^= (x<< 7) & 0x9d2c5680;"
    x ^= (x<<7) & 0x00001680;
    x ^= (x<<7) & 0x000c4000;
    x ^= (x<<7) & 0x0d200000;
    x ^= (x<<7) & 0x90000000;

    // reverse "x ^=  x>>11;"
    x ^= x>>11;
    x ^= x>>22;

    x
}
