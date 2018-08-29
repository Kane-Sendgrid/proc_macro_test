#![feature(proc_macro_non_items)]

extern crate proc_macro_test;

use proc_macro_test::test;

static A: [bool; 256] = test!("!#$%&'*+-/=?^_`{|}~");

fn main() {
    println!("Hello, world!, {}, {}", A[0], A[33]);
}
