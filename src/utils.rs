extern crate num;
extern crate alloc;
extern crate rgsl;

use heapslice;
use self::num::Float;
// use std::ops;
use std::f32;
use std::f64;

// you can't do std::<T>::consts::PI. Workaround (needed e.g. for sinc())
// https://github.com/rust-lang/rfcs/pull/1062
// http://stackoverflow.com/questions/32763783/how-to-access-numeric-constants-using-the-float-trait

pub trait Castd {
    fn d(self: &Self) -> f64;
}

impl Castd for f32 {
    fn d(self: &Self) -> f64 {*self as f64}
}

impl Castd for i32 {
    fn d(self: &Self) -> f64 {*self as f64}
}

pub trait FloatConst {
    fn pi() -> Self;
}

impl FloatConst for f32 {
    fn pi() -> Self { f32::consts::PI }
}

impl FloatConst for f64 {
    fn pi() -> Self { f64::consts::PI }
}


pub fn linspace_vec<'a, T: 'a>(start: T, stop: T, len: usize) ->
Vec<T>
    where T: Float {

    let zero: T = T::zero();
    let one: T = T::one();
    let len_t_minus1: T = num::cast(len-1).unwrap();
    let dx = (stop-start)/len_t_minus1;

    // let mut v = Vec::<T>::with_capacity(len);
    //
    // for i in 0..len {
    //     v.push(zero);
    // }

    let mut v = vec![zero; len];

    let mut c = zero;

    //**** SLOW ****
    // for x in v.iter_mut() {
    //     *x = start + c*dx;
    //     c = c + one;
    // }

    //**** FAST ****
    let ptr: *mut T = v.as_mut_ptr();
    unsafe {
        for ii in 0..len {
            let x = ptr.offset((ii as isize));
            *x = start + c*dx;
            c = c + one;
        }
    }

    return v
}



pub fn kaiser<T>(length: usize, alpha: T) -> heapslice::HeapSlice<T> where T: Float+FloatConst {
    let length_t: T = num::cast(length).unwrap();
    let one = T::one();
    let two: T = num::cast(2).unwrap();
    let mut n = linspace_heapslice(T::zero(), (length_t-one), length);
    for ni in n.iter_mut() {
        let mut tmp= two*(*ni)/(length_t-one)-one;
        tmp = T::pi()*alpha* ( one- tmp.powf(two) ).sqrt();
        let tmpf64: f64 = num::cast(tmp).unwrap();
        let grr: f64 = num::cast(T::pi()*alpha).unwrap();
        let tmp2 = rgsl::bessel::I0(tmpf64) / rgsl::bessel::I0( grr );
        *ni=num::cast(tmp2).unwrap();
    }
    return n
}


pub fn linspace_heapslice<'a, T: 'a>(start: T, stop: T, len: usize) -> heapslice::HeapSlice<T>
    where T: Float {

    let zero: T = T::zero();
    let one: T = T::one();
    let len_t_minus1: T = num::cast(len-1).unwrap();
    let dx = (stop-start)/len_t_minus1;

    let mut fb: heapslice::HeapSlice<T> = heapslice::HeapSlice::<T>::new();
    fb.allocate(len);

    unsafe {
        let mut c = zero;

        for ii in 0..len {
            let x = fb.ptr.offset((ii as isize));
            *x = start + c*dx;
            c = c + one;
        }

        return fb
    }
}




// pub fn sinc(f64) -> HeapSlice<T> {
//     let mut fb: HeapSlice<T> = HeapSlice::<T>::new();
//     fb.allocate(self.length);
//     for (xout,xin) in &mut fb.iter_mut().zip(self.iter()) {
//         if *xin != T::zero() {
//             *xout = (*xin*T::pi()).sin()/(*xin*T::pi());
//         } else {
//             *xout = T::one()
//         }
//     }
//     fb
// }
