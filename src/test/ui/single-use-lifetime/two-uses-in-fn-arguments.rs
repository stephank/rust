// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that we DO NOT warn when lifetime name is used multiple
// argments, or more than once in a single argument.
//
// compile-pass

#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(unused_variables)]

fn c<'a>(x: &'a u32, y: &'a u32) { // OK: used twice
}

fn d<'a>(x: (&'a u32, &'a u32)) { // OK: used twice
}

fn main() { }
