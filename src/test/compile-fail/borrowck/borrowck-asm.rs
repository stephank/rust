// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-s390x
// ignore-emscripten
// ignore-powerpc
// ignore-sparc

// revisions: ast mir
//[mir]compile-flags: -Z borrowck=mir

#![feature(asm)]

#[cfg(any(target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "mips",
            target_arch = "mips64"))]
mod test_cases {
    fn is_move() {
        let y: &mut isize;
        let x = &mut 0isize;
        unsafe {
            asm!("nop" : : "r"(x));
        }
        let z = x;  //[ast]~ ERROR use of moved value: `x`
                    //[mir]~^ ERROR use of moved value: `x`
    }

    fn in_is_read() {
        let mut x = 3;
        let y = &mut x;
        unsafe {
            asm!("nop" : : "r"(x)); //[ast]~ ERROR cannot use
                                    //[mir]~^ ERROR cannot use
        }
        let z = y;
    }

    fn out_is_assign() {
        let x = 3;
        unsafe {
            asm!("nop" : "=r"(x));  //[ast]~ ERROR cannot assign twice
                                    //[mir]~^ ERROR cannot assign twice
        }
        let mut a = &mut 3;
        let b = &*a;
        unsafe {
            asm!("nop" : "=r"(a));  //[ast]~ ERROR cannot assign to `a` because it is borrowed
                                    // No MIR error, this is a shallow write.
        }
        let c = b;
        let d = *a;
    }

    fn rw_is_assign() {
        let x = 3;
        unsafe {
            asm!("nop" : "+r"(x));  //[ast]~ ERROR cannot assign twice
                                    //[mir]~^ ERROR cannot assign twice
        }
    }

    fn indirect_is_not_init() {
        let x: i32;
        unsafe {
            asm!("nop" : "=*r"(x)); //[ast]~ ERROR use of possibly uninitialized variable
                                    //[mir]~^ ERROR use of possibly uninitialized variable
        }
    }

    fn rw_is_read() {
        let mut x = &mut 3;
        let y = &*x;
        unsafe {
            asm!("nop" : "+r"(x));  //[ast]~ ERROR cannot assign to `x` because it is borrowed
                                    //[mir]~^ ERROR cannot assign to `x` because it is borrowed
        }
        let z = y;
    }

    fn two_moves() {
        let x = &mut 2;
        unsafe {
            asm!("nop" : : "r"(x), "r"(x) );    //[ast]~ ERROR use of moved value
                                                //[mir]~^ ERROR use of moved value
        }
    }
}

fn main() {}
