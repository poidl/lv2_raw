#![allow(unused_variables)]
extern crate yassy;
extern crate rgsl;
extern crate gnuplot;

use yassy::utils;

fn main() {
    let pi = std::f64::consts::PI;
    let x = utils::linspace_fastbox(-1f64, 1f64, 100);
    let x = x*2f64*pi;
    let y = x.sinc();

    for ii in x.iter() {
        println!("ii: {}",ii)
    }

    let mut fg = gnuplot::Figure::new();
    fg.set_terminal("svg","./examples/hoit.svg");
    fg.axes2d()
    .lines(x.iter(), y.iter(), &[]);
    fg.show();
}
