#![allow(unused_variables)]

#![feature(test)]

extern crate test; // for bench

extern crate yassy;
// extern crate alloc;

use test::Bencher;
use yassy::utils;
// use alloc::heap;

const  LENGTH: usize = 1000000;

#[bench]
fn bench1(b: &mut Bencher) {
    b.iter(|| utils::linspace_vec(1f64, 5f64, LENGTH));
}
//
#[bench]
fn bench2(b: &mut Bencher) {
    b.iter(|| utils::linspace_vec2box(1f64, 5f64, LENGTH));
}

//
// #[bench]
// fn bench3(b: &mut Bencher) {
//     b.iter(|| utils::linspace_slice(1f64, 5f64, LENGTH));
//     heap::stats_print();
// }
//
// // NEED TO IMPLEMENT DROP. MEMORY NEVER FREED.
//
// #[bench]
// fn bench4(b: &mut Bencher) {
//     b.iter(|| utils::linspace_slice_unchecked(1f64, 5f64, LENGTH));
// }
//
// #[bench]
// fn bench5(b: &mut Bencher) {
//     b.iter(|| utils::linspace_ptr(1f64, 5f64, LENGTH));
// }

#[bench]
fn bench6(b: &mut Bencher) {
    b.iter(|| utils::linspace_fastbox(1f64, 5f64, LENGTH));
}

// #[bench]
// fn bench7(b: &mut Bencher) {
//     b.iter(|| utils::linspace_boxed_slice(1f64, 5f64, LENGTH));
// }
