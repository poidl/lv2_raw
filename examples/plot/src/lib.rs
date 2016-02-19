extern crate yassy;
extern crate gnuplot;

use yassy::utils;
use yassy::utils::*;
use self::gnuplot::*;

pub fn plot_ampl_spec(nt: usize, nppt: usize, nn: usize, fs: f64, fhabs: &[f64], outname: &str) {

    // The axis of fhabs has nn/2+1 points, representing frequencies from 0 to fl/2,
    // or i*(fl/2)/(nn/2) = i*fl/nn = i*fs*(nppt-1)/nn for i=0..nn/2. (Because
    // fl=1/Tl=fs*(nppt-1)) We are only interested in
    // frequencies up to around fi=60KHz, or i= 60KHz*nn/(fs*(nppt-1)).

    let npptf64=nppt as f64;
    let ntf64=nt as f64;
    // Find index such that the horizontal axis of the plot is fmax, i.e.
    // i = fmax*nn/(fs*(nppt-1))
    let fac = (nn as f64)/(fs*(npptf64-1f64));
    let i_fi = (60000f64*fac).round();
    println!("fac: {}", fac);
    println!("i_fi: {}", i_fi);

    let mut f = vec![0f64; nn/2+1];
    // display kHz in plot
    utils::linspace(&mut f, 0f64, ((nn/2+1) as f64)/fac/1000f64);
    let f_cut = &f[..i_fi as usize];
    let fhabs_cut = &fhabs[..i_fi as usize];

    let mut fg = gnuplot::Figure::new();

    fg.set_terminal("svg", outname);
    // yticks
    let yticks = [0.00001f64,0.0001f64,0.001f64,0.01f64,0.1f64,1f64];
    fg.axes2d()
    .set_y_log(Some(10f64))
    .lines(f_cut.iter(), fhabs_cut.iter(), &[Color("blue")])
    .lines(&[20f64,20f64], &[0f64, 1f64], &[Color("green")])
    .lines(&[fs/1000f64,fs/1000f64], &[0f64, 1f64], &[Color("red")])
    .lines(&[fs/1000f64-20f64,fs/1000f64-20f64], &[0f64, 1f64], &[Color("red")])
    .set_y_range(Fix(0.00001f64), Fix(1f64))
    .set_y_ticks_custom(yticks.iter().map(|x| Major(*x as f64, Fix("10^{%T}".to_string()))),&[],&[])
    .set_x_label("Frequency in kHz",&[])
    .set_title("Amplitude spectrum",&[]);
    fg.show();

}
