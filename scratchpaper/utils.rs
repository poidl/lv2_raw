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

// **********************************************

// pub trait ToSinc<T> {
//     fn sinc(self: &mut Self) -> &mut Self;
// }

// pub trait TestTrait: Sized {
//     fn test_in_place(&mut self);
//     fn test(mut self) -> Self {
//         self.test_in_place();
//         self
//     }
// }
//
// impl<'a> TestTrait for &'a mut [i32] {
//     fn test_in_place(self: &mut Self) {
//         for x in (**self).iter_mut() {
//             *x=*x*3;
//         }
//     }
//     // fn test(self: mut Self) -> Self {
//     //     self.test_in
//     // }
// }


// pub fn test(x: i32) -> i32 {
//     let y=2*x;
//     y
// }
//
// pub fn test2(x: Vec<i32>) {
//     let y=x[0];
// }

pub trait ToSinc {
    fn sinc(self: Self) -> Self;
}

// **********************************************

// pub fn sinc<T: ToSinc<T>>(y: &mut T) -> &mut T {
//     y.sinc()
// }

pub fn sinc<T: ToSinc>(y: T) -> T {
    y.sinc()
}

// **********************************************

// impl ToSinc<f64> for f64 {
//     fn sinc(self: &mut Self) -> &mut Self {
//         if *self != 0f64 {
//             *self = (*self*f64::consts::PI).sin()/(*self*f64::consts::PI);
//         } else {
//             *self = 1f64;
//         }
//         self
//     }
// }

impl<'a> ToSinc for f64 {
    fn sinc(self) -> Self {
        let new: f64;
        if self != 0f64 {
            let new = (self*f64::consts::PI).sin()/(self*f64::consts::PI);
            return new
        } else {
            let new = 1f64;
            return new
        }
    }
}

// **********************************************

// impl<'a> ToSinc<&'a mut [f64]> for &'a mut [f64] {
//     fn sinc(self: &mut Self) -> &mut Self {
//         for yi in (**self).iter_mut() {
//             if *yi != 0f64 {
//                 *yi = (*yi*f64::consts::PI).sin()/(*yi*f64::consts::PI);
//             } else {
//                 *yi = 1f64;
//             }
//         }
//         self
//     }
// }

impl<'a> ToSinc for &'a mut [f64] {
    fn sinc(self) -> Self {
        for yi in (*self).iter_mut() {
            if *yi != 0f64 {
                *yi = (*yi*f64::consts::PI).sin()/(*yi*f64::consts::PI);
            } else {
                *yi = 1f64;
            }
        }
        self
    }
}



// **********************************************

// pub fn sinc2(y: &mut f64) -> &mut f64 {
//     if *y != 0f64 {
//         *y = (*y*f64::consts::PI).sin()/(*y*f64::pi());
//     } else {
//         *y = 1f64;
//     }
//     y
// }
//
// pub fn sinc(y: &mut [f64]) -> &mut [f64] {
//     for yi in y.iter_mut() {
//         if *yi != 0f64 {
//             *yi = (*yi*f64::consts::PI).sin()/(*yi*f64::consts::PI);
//         } else {
//             *yi = 1f64;
//         }
//     }
//     y
// }

// pub fn linspace(y: &mut [f64]) -> &mut [f64] {
//     for yi in y.iter_mut() {
//         if *yi != 0f64 {
//             *yi = (*yi*f64::pi()).sin()/(*yi*f64::pi());
//         } else {
//             *yi = 1f64;
//         }
//     }
//     y
// }
