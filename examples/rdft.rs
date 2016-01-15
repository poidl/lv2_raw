#![allow(unused_variables)]
extern crate yassy;
extern crate rgsl;
extern crate gnuplot;

// use std::ops::{Deref, DerefMut};

use yassy::utils;
use gnuplot::*;

fn main() {
    let pi = std::f64::consts::PI;

    // Each sampling interval T is represented by exactly nppt points (number of
    // points per T). Denoting the resolution of the temporal lattice as Tl and
    // the corresponding frequency as fl, it follows that T=(nppt-1)*Tl, and
    // fs=1/T=fl/(nppt-1), where fl=1/Tl.
    // nt is number of sampling intervals T on the temporal lattice, whose length
    // is n=nt*nppt-1. Note that a point representing the end of one T represents
    // also the starting point of the next T.
    // The time signal is symmetric around 0, and if we want 0 to coincide
    // exactly with a lattice point, then nt must be an even number. It follows
    // from n=nt*(nppt-1)+1 that n must be uneven. This is somewhat inconvenient,
    // because the most efficient fft algorithms only accept vectors with a
    // length that is a power of 2 (and hence is even). We therefore append
    // some points to the time series until it becomes a power of 2. This will
    // increase the frequency resolution. See also the book of Frei, especially
    // the appendix with the MATLAB code listing.
    // Frei, B.: Digital sound generation. Institute for Computer Music and Sound Technology (ICST) Zurich University of the Arts.

    let nt = 10;
    let nppt = 4;
    let n = nt*(nppt-1)+1 as usize;
    let nn=2u32.pow(12u32) as usize;
    let fs = 48000f64; // sampling frequency 1/T.
    let fc = 20000f64; // cutoff frequency 1/Tc.

    let alpha = 3f64; // alpha for Kaiser window

    let nth = (nt as f64)*0.5f64;
    let t = utils::linspace_heapslice(-nth , nth , n);

    // The first zero crossing of the impulse determines the cutoff frequency fc=(1/Tc).
    // The normalized sinc function sinc(T) has the first zero crossing at 1*T,
    // corresponding to a cutoff at 0.5(1/T)=0.5*fs, which differs by a factor of
    // c=2*fc/fs from fc.  To get the cutoff at fc, we use c*sinc(c*T) (see doc for
    // the effect of linear axis scaling on the Fourier transform).

    let c = 2f64*fc/fs;

    // impulse h
    let h = (1f64/(nppt-1) as f64)*c*utils::sinc( c*t.clone() );

    // let kaiser = utils::kaiser(n,alpha);
    //
    // let hk = h.clone()*kaiser.clone();
    let hk=h.clone();
    // Before Fourier transforming h to fh, append points to make the length a
    // power of 2
    let mut fh = utils::linspace_heapslice(0f64, 1f64 , nn);
    fh = 0f64*fh;
    for i in 0..hk.len()-1 {
        fh[i]=hk[i];
    }
    rgsl::fft::real_radix2::transform(&mut fh,1,nn);
    // println!("err: {}", rgsl::error::str_error(err));
    //rgsl::fft::mixed_radix::transform(&mut fh,1,n,[],[],[]);

    // let mut hh_padded = utils::linspace_heapslice( 0f64, 1f64, n+20*n);
    // for i in 0..hh_padded.len()-1 {
    //     if i<n {
    //         hh_padded[i]=hh[i];
    //     } else {
    //         hh_padded[i]=0f64;
    //     }
    // }
    // rgsl::fft::real_radix2::transform(&mut hh_padded,1,n+20*n);

    // magnitude (abs) of Re and Im
    let mut fhabs = utils::linspace_heapslice(-1f64, 1f64, nn/2 +1);
    for ii in 0..nn/2 {
        fhabs[ii]=(fh[ii].powf(2f64)+fh[nn-1-ii].powf(2f64)).sqrt();
    }

    // let mut mag_hh_padded = utils::linspace_heapslice(-1f64, 1f64, n/2 +1);
    // for ii in 0..l/2 {
    //     mag_hh_padded[ii]=(hh_padded[ii].powf(2f64)+hh_padded[l-1-ii].powf(2f64)).sqrt();
    // }



    let mut fg = gnuplot::Figure::new();
    fg.set_terminal("svg","./examples/h.svg");
    fg.axes2d()
    .lines(t.iter(), h.iter(), &[]);
    fg.show();

    // let mut fg = gnuplot::Figure::new();
    // fg.set_terminal("svg","./examples/kaiser.svg");
    // fg.axes2d()
    // .lines(t.iter(), kaiser.iter(), &[]);
    // fg.show();

    let mut fg = gnuplot::Figure::new();
    fg.set_terminal("svg","./examples/hk.svg");
    fg.axes2d()
    .lines(t.iter(), hk.iter(), &[]);
    fg.show();

    // let mut fg = gnuplot::Figure::new();
    // fg.set_terminal("svg","./examples/fh.svg");
    // fg.axes2d()
    // .lines(.iter(), fh.iter(), &[]);
    // fg.show();

    // let mut fg = gnuplot::Figure::new();
    // fg.set_terminal("svg","./examples/hh.svg");
    // fg.axes2d()
    // .lines(x_padded.iter(), hh_padded.iter(), &[]);
    // fg.show();

    // The axis of fhabs has nn/2+1 points, representing frequencies from 0 to fl/2,
    // or i*(fl/2)/(nn/2) = i*fl/nn = i*fs*(nppt-1)/nn for i=0..nn/2. (Because
    // fl=1/Tl=fs*(nppt-1)) We are only interested in
    // frequencies up to around fi=60KHz, or i= 60KHz*nn/(fs*(nppt-1)).

    // print out some usuful numbers
    let npptf64 = nppt as f64;
    println!("n:    {}", n);
    println!("fs:   {}", fs);
    println!("nppt: {}", nppt);
    println!("fl:   {}", fs*(npptf64-1f64));
    println!("fl/2: {}", fs*(npptf64-1f64)/2f64);

    let nf64=n as f64;
    let ntf64=nt as f64;
    // Find index such that the horizontal axis of the plot is fmax, i.e.
    // i = fmax*nn/(fs*(nppt-1))
    let fac = (nn as f64)/(fs*(npptf64-1f64));
    let i_fi = (60000f64*fac).round();
    println!("fac: {}", fac);
    println!("i_fi: {}", i_fi);

    let f = utils::linspace_heapslice(0f64, i_fi/fac , i_fi as usize);
    let fhabs_cut = &fhabs[..i_fi as usize];

    let mut fg = gnuplot::Figure::new();
    fg.set_terminal("svg","./examples/fhabs.svg");
    fg.axes2d()
    .set_y_log(Some(10f64))
    .lines(f.iter(), fhabs_cut.iter(), &[]);
    fg.show();

}
