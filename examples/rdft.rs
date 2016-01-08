#![allow(unused_variables)]
extern crate yassy;
extern crate rgsl;
extern crate gnuplot;

// use std::ops::{Deref, DerefMut};

use yassy::utils;
use gnuplot::*;

fn main() {
    let pi = std::f64::consts::PI;

    // nt is number of sampling intervals T on time axis, the signal within each T is
    // resolved by nttp points. n is length of time axis in points. Time axis t is
    // symmetric around 0.
    let nt = 4;
    let nppt = 2;
    let n = (4*2+1) as usize;

    let fs = 48000f64; // sampling frequency 1/T.
    let fc = 20000f64; // cutoff frequency 1/Tc.

    let alpha = 9f64; // alpha for Kaiser window

    let nth = (nt as f64)*0.5f64;
    let t = utils::linspace_heapslice(-nth , nth , n);

    // The first zero crossing of the impulse determines the cutoff frequency fc=(1/Tc).
    // The normalized sinc function sinc(T) has the first zero crossing at 1*T,
    // corresponding to a cutoff at 0.5(1/T)=0.5*fs, which differs by a factor of
    // 2*T*fc = 2*fc/fs from fc.  To get the cutoff at fc, we use c*sinc(c*T), where
    // c=1/(2*T*fc)=fs/(2*fc) (see doc for the effect of linear axis scaling on the
    // Fourier transform).

    let c = fs/(2f64*fc);

    // impulse h
    let h = c * utils::sinc( c*t.clone() );

    // let kaiser = utils::kaiser(n,8.3);
    let kaiser = utils::kaiser(n,alpha);

    let hk = h.clone()*kaiser.clone();

    // Fourier transform h to fh
    let mut fh = hk.clone();
    //rgsl::fft::real_radix2::transform(&mut fh,1,n);
    rgsl::fft::mixed_radix::transform(&mut fh,1,n,[],[],[]);

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
    let mut fhabs = utils::linspace_heapslice(-1f64, 1f64, n/2 +1);
    for ii in 0..n/2 {
        fhabs[ii]=(fh[ii].powf(2f64)+fh[n-1-ii].powf(2f64)).sqrt();
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

    let mut fg = gnuplot::Figure::new();
    fg.set_terminal("svg","./examples/kaiser.svg");
    fg.axes2d()
    .lines(t.iter(), kaiser.iter(), &[]);
    fg.show();

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

    let x = utils::linspace_heapslice(0f64, 0.5f64, n/2+1);

    // The signal within each T is resolved by nppt=n/nt points per T. Denoting the temporal
    // resolution of the time axis as Tr=T/nppt, the corresponding "resolution frequency" is
    // fr=1/Tr. The axis of fhabs has n/2+1 points, representing frequencies from 0 to fr/2,
    // or i*(fr/2)/(n/2) = i*fr/n = i*fs/nt for i=0..n/2. We are only interested in
    // frequencies up to around fi=60KHz, or i= (nt/fs)*60KHz.

    // print out some usuful numbers
    let npptf64 = nppt as f64;
    let tr = (1f64/fs)/npptf64;
    println!("n:    {}", n);
    println!("fs:   {}", fs);
    println!("T:    {}", 1f64/fs);
    println!("nppt: {}", nppt);
    println!("fr:   {}", 1f64/tr);
    println!("Tr:   {}", tr);
    println!("fr/2: {}", 1f64/(tr*2f64));

    let nf64=n as f64;
    let ntf64=nt as f64;
    let fac = ntf64/fs;
    // Find index such that the horizontal axis of the plot is fmax, i.e.
    // i = (nt/fs)*fmax
    let i_fi = 5f64;// (fac*60000f64).round();
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
