extern crate rgsl;

use std::f64;

pub fn linspace(slice: &mut [f64], start: f64, stop: f64) -> &mut [f64] {

    let dx = (stop-start)/((slice.len()-1) as f64);
    let mut c :i32 =0;
    unsafe {
        for ii in 0..slice.len() {
            let x = slice.get_unchecked_mut(ii);
            *x = start + (c as f64)*dx;
            c = c + 1;
        }
        return slice
    }
}

pub trait Mult<RHS: ?Sized> {
    fn mult(self: &mut Self, rhs: &RHS);
}

impl Mult<[f64]> for [f64]  {
    fn mult(self: &mut Self, rhs: &Self) {
        for ii in 0..self.len() {
            unsafe {
                let x = self.get_unchecked_mut(ii);
                let y = rhs.get_unchecked(ii);
                *x=*x*(*y);
            }
        }
    }
}


impl Mult<f64> for [f64] {
    fn mult(self: &mut Self, c: &f64) {
        for ii in 0..self.len() {
            unsafe {
                let x = self.get_unchecked_mut(ii);
                *x=*x*c;
            }
        }
    }
}

pub trait Mycopy {
    fn mycopy(self: &mut Self, rhs: &Self);
}

impl  Mycopy for [f64] {
    fn mycopy(self: &mut Self, rhs: &Self) {
        for ii in 0..self.len() {
            unsafe {
                let x = self.get_unchecked_mut(ii);
                let y = rhs.get_unchecked(ii);
                *x=*y;
            }
        }
    }
}

// pub fn mult_inplace(c: f64, slice: &mut [f64]) -> &mut [f64] {
//     for ii in 0..slice.len() {
//         unsafe {
//             let x = slice.get_unchecked_mut(ii);
//             *x=*x*c;
//         }
//     }
//     slice
// }

fn sinc_element(x: &mut f64) -> &mut f64 {
    if *x != 0f64 {
        *x = (*x*f64::consts::PI).sin()/(*x*f64::consts::PI);
    } else {
        *x = 1f64;
    }
    x
}

pub trait Sinc {
    fn sinc(self: &mut Self);
}

impl Sinc for [f64] {
    fn sinc(self: &mut Self) {
        unsafe {
            for ii in 0..self.len() {
                let x = self.get_unchecked_mut(ii);
                sinc_element(x);
            }
        }
    }
}

fn kaiser_element(n: u32, alpha: f64, len: u32) -> f64 {
    let pi = f64::consts::PI;
    let nf = n as f64;
    let lenf = len as f64;
    let mut tmp= 2f64*nf/(lenf-1f64)-1f64;
    tmp = pi*alpha* ( 1f64- tmp.powf(2f64) ).sqrt();
    rgsl::bessel::I0(tmp) / rgsl::bessel::I0( pi*alpha )
}

pub trait Kaiser {
    fn kaiser(self: &mut Self, alpha: f64);
}

impl Kaiser for [f64] {
    fn kaiser(self: &mut Self, alpha: f64) {
        for ii in 0..self.len() {
            let kaiser = kaiser_element(ii as u32,alpha, self.len() as u32);
            unsafe {
                let x = self.get_unchecked_mut(ii);
                *x=*x*kaiser;
            }
        }
    }
}
