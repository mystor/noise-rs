// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use noise_fns::NoiseFn;

/// Noise function that inverts the output value from the source function.
pub struct Invert<'a, T: 'a> {
    /// Outputs a value.
    pub source: &'a NoiseFn<T>,
}

impl<'a, T> Invert<'a, T> {
    pub fn new(source: &'a NoiseFn<T>) -> Invert<'a, T> {
        Invert { source: source }
    }
}

impl<'a, T> NoiseFn<T> for Invert<'a, T> {
    fn get(&self, point: T) -> f64 {
        -self.source.get(point)
    }
}
