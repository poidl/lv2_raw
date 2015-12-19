#![allow(unused_variables)]
extern crate yassy;
extern crate rgsl;
extern crate gnuplot;

// use std::ops::{Deref, DerefMut};

use yassy::utils;


fn main() {
    let pi = std::f64::consts::PI;
    let n = 2u32.pow(8) as usize;
    let x = utils::linspace_heapslice(-1f64, 1f64, n);
    let xt = x.clone();
    let fc = 20f64;
    let  y = (2f64*fc)*(2f64*fc*xt).sinc();

    // for ii in x.iter() {
    //     println!("ii: {}",ii)
    // }
    // let z = y.copy();
    let l = y.len();
    println!("length: {}", l);
    // let val = rgsl::fft::real_radix2::transform(&mut y,1,l);
    // let xx = utils::linspace_heapslice(0f64, (n/2+1) as f64, n/2+1);

// TODO: *) understand copy and clone
    // TODO inefficient:
    let mut mag = utils::linspace_heapslice(-1f64, 1f64, n/2 +1);
    for ii in 0..l/2 {
        mag[ii]=(y[ii].powf(2f64)+y[l-1-ii].powf(2f64)).sqrt();
    }
    
    let mut fg = gnuplot::Figure::new();
    fg.set_terminal("svg","./examples/hoit.svg");
    fg.axes2d()
    .lines(x.iter(), y.iter(), &[]);
    fg.show();

    let x = utils::linspace_heapslice(0f64, (n/2) as f64, n/2+1);
    let mut fg = gnuplot::Figure::new();
    fg.set_terminal("svg","./examples/hoit2.svg");
    fg.axes2d()
    .lines(x.iter(), mag.iter(), &[]);
    fg.show();


    let x = utils::linspace_heapslice(0f64, (n-1) as f64, n);
    let kaiser = utils::kaiser(n,8.3);
    let mut fg = gnuplot::Figure::new();
    fg.set_terminal("svg","./examples/kaiser.svg");
    fg.axes2d()
    .lines(x.iter(), kaiser.iter(), &[]);
    fg.show();
    // let mut fg = gnuplot::Figure::new();
    // fg.set_terminal("svg","./examples/hoit.svg");
    // fg.axes2d()
    // .lines(x.iter(), yy.iter(), &[]);
    // fg.show();


}
