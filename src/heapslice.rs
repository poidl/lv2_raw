extern crate num;
extern crate alloc;
extern crate rgsl;

use utils;
use self::num::Float;
use std::mem;

use self::alloc::heap;
use std::slice;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops;
use std::ptr::{Unique, self};
use std::f64;

use utils::ToSinc;


// similar to Vec as described in Rustonomicon
pub struct HeapSlice<T> {
    // ptr: *mut T,
    pub ptr: Unique<T>,
    length: usize,
    // marker: marker::PhantomData<*mut T>,
}

fn oom() {
    ::std::process::exit(-9999);
}

// #![feature(alloc, heap_api)]

impl<T> HeapSlice<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        unsafe {
            // need to cast EMPTY to the actual ptr type we want, let
            // inference handle it.
            HeapSlice { ptr: Unique::new(heap::EMPTY as *mut _), length: 0 }
        }
    }

    pub fn allocate(&mut self, newlength: usize) {
        unsafe {
            let typesize = mem::size_of::<T>();
            let align = mem::align_of::<T>();
            let size = newlength * typesize;

            if self.length == 0 {
                let ptr = heap::allocate(size, align);
                // If allocate fails, we'll get `null` back
                if ptr.is_null() { oom(); }
                self.ptr = Unique::new(ptr as *mut _);
                self.length = newlength;
            } else {
                panic!("already allocated ?")
            }
        }
    }

}

impl<T> Drop for HeapSlice<T> {
    fn drop(&mut self) {
        if self.length!= 0 {
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let size = elem_size * self.length;
            unsafe {
                heap::deallocate(*self.ptr as *mut u8, size, align)
            }
        }
    }
}

impl<T: Copy> Clone for HeapSlice<T> {
    fn clone(&self) -> HeapSlice<T> {
        to_heapslice(&**self)
    }
}

pub fn to_heapslice<T>(s: &[T]) -> HeapSlice<T>
    where T: Copy
{
    let mut fb: HeapSlice<T> = HeapSlice::<T>::new();
    fb.allocate(s.len());
    // // from Vec's extend_from_slice. necessary, useful?
    // unsafe {
    //     for i in 0..s.len() {
    //         ptr::write(fb.get_unchecked_mut(i), s.get_unchecked(i).clone());
    //     }
    // }
    for i in 0..s.len() {
        fb[i]=s[i];
    }
    fb
}


impl<T> Deref for HeapSlice<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(*self.ptr, self.length)
        }
    }
}

impl<T> DerefMut for HeapSlice<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(*self.ptr, self.length)
        }
    }
}


impl<T> ops::Mul<T> for HeapSlice<T> where T: Float {
    type Output = HeapSlice<T>;
    fn mul(self, f: T) -> HeapSlice<T> {
        let mut fb: HeapSlice<T> = HeapSlice::<T>::new();
        fb.allocate(self.length);
        for (xout,xin) in &mut fb.iter_mut().zip(self.iter()) {
            *xout = f*(*xin);
        }
        fb
    }
}

impl ops::Mul<HeapSlice<f64>> for f64 {
    type Output = HeapSlice<f64>;
    fn mul(self, f: HeapSlice<f64>) -> HeapSlice<f64> {
        let mut fb: HeapSlice<f64> = HeapSlice::<f64>::new();
        fb.allocate(f.length);
        for (xout,xin) in &mut fb.iter_mut().zip(f.iter()) {
            *xout = self*(*xin);
        }
        fb
    }
}



impl ops::Mul<i32> for HeapSlice<f64>  {
    type Output = HeapSlice<f64>;
    fn mul(self, f: i32) -> HeapSlice<f64> {
        let mut fb: HeapSlice<f64> = HeapSlice::<f64>::new();
        fb.allocate(self.length);
        for (xout,xin) in &mut fb.iter_mut().zip(self.iter()) {
            *xout = (f as f64)*(*xin);
        }
        fb
    }
}

impl ops::Mul<HeapSlice<f64>> for HeapSlice<f64>  {
    type Output = HeapSlice<f64>;
    fn mul(self, f: HeapSlice<f64>) -> HeapSlice<f64> {
        let mut fb: HeapSlice<f64> = HeapSlice::<f64>::new();
        fb.allocate(self.length);
        unsafe {
            for i in 0..fb.len() {
                ptr::write(fb.get_unchecked_mut(i), self.get_unchecked(i).clone() * f.get_unchecked(i).clone());
            }
        }
        fb
    }
}


impl<T> HeapSlice<T> where T: Float {
    pub fn sin(&self) -> HeapSlice<T> {
        let mut fb: HeapSlice<T> = HeapSlice::<T>::new();
        fb.allocate(self.length);
        for (xout,xin) in &mut fb.iter_mut().zip(self.iter()) {
            *xout = (*xin).sin();
        }
        fb
    }
}

impl ToSinc for HeapSlice<f64> {
    fn sinc(self) -> Self {
        let mut y = self;
        let ga = utils::sinc(&mut *y);
        to_heapslice(ga)
    }
}

// impl<T> HeapSlice<T> where T: Float+utils::FloatConst {
//     pub fn sinc(mut self) -> HeapSlice<T> {
//         for xout in self.iter_mut() {
//             if *xout != T::zero() {
//                 *xout = (*xout*T::pi()).sin()/(*xout*T::pi());
//             } else {
//                 *xout = T::one()
//             }
//         }
//         self
//     }
// }
