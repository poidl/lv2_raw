#![feature(test)]

extern crate test; // for bench

extern crate yassy;

use test::Bencher;
use yassy::utils;

#[bench]
fn bench1(b: &mut Bencher) {
    b.iter(|| utils::linspace_vec(1f64, 5f64, 1000000));
}

#[bench]
fn bench2(b: &mut Bencher) {
    b.iter(|| utils::linspace_vec2box(1f64, 5f64, 1000000));
}

#[bench]
fn bench3(b: &mut Bencher) {
    b.iter(|| utils::linspace_slice(1f64, 5f64, 1000000));
}

#[bench]
fn bench4(b: &mut Bencher) {
    b.iter(|| utils::linspace_ptr(1f64, 5f64, 1000000));
}
