#![allow(unused_variables)]
extern crate yassy;
extern crate rgsl;
extern crate gnuplot;

// use std::ops::{Deref, DerefMut};

use yassy::utils;

fn main() {
    let pi = std::f64::consts::PI;
    let n = 2u32.pow(5) as usize;
    let x = utils::linspace_heapslice(-1f64, 1f64, n);
    let x = x*2f64*pi;
    let mut y = x.sinc();

    // for ii in x.iter() {
    //     println!("ii: {}",ii)
    // }
    // let z = y.copy();
    let l = y.len();
    println!("length: {}", l);
    let val = rgsl::fft::real_radix2::transform(&mut y,1,l);
    // let xx = utils::linspace_heapslice(0f64, (n/2+1) as f64, n/2+1);

// TODO: *) understand copy and clone

    let mut fg = gnuplot::Figure::new();
    fg.set_terminal("svg","./examples/hoit.svg");
    fg.axes2d()
    .lines(x.iter(), y.iter(), &[]);
    fg.show();

    // let mut fg = gnuplot::Figure::new();
    // fg.set_terminal("svg","./examples/hoit.svg");
    // fg.axes2d()
    // .lines(x.iter(), yy.iter(), &[]);
    // fg.show();


}
