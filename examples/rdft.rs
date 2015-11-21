#![allow(unused_variables)]
extern crate yassy;
extern crate rgsl;
extern crate gnuplot;

use yassy::utils;

fn main() {
    let pi = std::f64::consts::PI;
    let mut v1 = utils::linspace_vec(-1f64, 1f64, 100);
    for x in &mut v1 {
        *x = *x*pi
    }
    println!("Hello, world!");

    let v2 = utils::linspace_fastbox(-1f64, 1f64, 10);
    for ii in v2.iter() {
        println!("v2: {}", ii);
    }

    let hoi = v2*3f64;
    for ii in hoi.iter() {
        println!("v3: {}", ii);
    }

}
