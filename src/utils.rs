extern crate num;
extern crate alloc;

use self::num::NumCast;
use self::num::Float;
use std::mem;

use self::alloc::heap;
use std::mem::{align_of, transmute};
use std::intrinsics;
use std::raw::Slice;
use std::i32;
use std::slice;
use std::marker;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops;

fn get_values_as_type_t<T>(start: T, stop: T, len: usize) -> (T, T, T)
    where T: Float {
    let zero: T = num::cast(0).unwrap();
    let len_t: T = num::cast(len).unwrap();
    let one: T = num::cast(1).unwrap();
    let diff = stop - start;
    let dx = diff/(len_t-one);
    return (one, zero, dx)
}

pub fn linspace_vec<'a, T: 'a>(start: T, stop: T, len: usize) ->
Vec<T>
    where T: Float {

    let (one, zero, dx) = get_values_as_type_t::<T>(start, stop, len);

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


pub fn linspace_vec2boxed_slice<'a, T: 'a>(start: T, stop: T, len: usize) -> Box<[T]>
    where T: Float {
    // get 0, 1 and the increment dx as T
    let (one, zero, dx) = get_values_as_type_t::<T>(start, stop, len);
    let mut v = vec![zero; len].into_boxed_slice();
    let mut c = zero;
    let ptr: *mut T = v.as_mut_ptr();
    unsafe {
        for ii in 0..len {
            let x = ptr.offset((ii as isize));
            *x = start + c*dx;
            c = c + one;
        }
    }

    v
}

pub fn make_arr_unsafe<'a, T>(len: usize) -> &'a mut [T] {

    let size = len * mem::size_of::<T>();

    unsafe {
        let ptr = heap::allocate(size, align_of::<T>());
        intrinsics::volatile_set_memory(ptr, 0, size);
        let slice = slice::from_raw_parts_mut(ptr as *mut T, len);
        return slice;
    }
}

pub fn linspace_slice<'a, T: 'a>(start: T, stop: T, len: usize) -> &'a [T]
    where T: Float {

    let (one, zero, dx) = get_values_as_type_t::<T>(start, stop, len);

    let size = len * mem::size_of::<T>();

    unsafe {
        let ptr = heap::allocate(size, align_of::<T>());
        let bx = slice::from_raw_parts_mut(ptr as *mut T, len);

        let mut c = zero;

        for x in bx.iter_mut() {
            *x = start + c*dx;
            c = c + one;
        }

        return bx
    }
}



pub fn linspace_slice_unchecked<'a, T: 'a>(start: T, stop: T, len: usize) -> &'a [T]
    where T: Float {

    let (one, zero, dx) = get_values_as_type_t::<T>(start, stop, len);

    let size = len * mem::size_of::<T>();

    unsafe {
        let ptr = heap::allocate(size, align_of::<T>());
        let bx = slice::from_raw_parts_mut(ptr as *mut T, len);

        let mut c = zero;

        for ii in 0..len {
            let x = bx.get_unchecked_mut(ii);
            *x = start + c*dx;
            c = c + one;
        }

        return bx
    }
}

pub fn linspace_ptr<'a, T: 'a>(start: T, stop: T, len: usize) -> *mut T
    where T: Float {

let (one, zero, dx) = get_values_as_type_t::<T>(start, stop, len);

    let size = len * mem::size_of::<T>();

    unsafe {
        let ptr = heap::allocate(size, align_of::<T>()) as *mut T;

        let mut c = zero;

        for ii in 0..len {
            let x = ptr.offset((ii as isize));
            *x = start + c*dx;
            c = c + one;
        }

        return ptr as *mut T
    }
}

// Similar to IntermediateBox
pub struct FastBox<T> {
    ptr: *mut T,
    length: usize,
    typesize: usize,
    size: usize,
    align: usize,
    // marker: marker::PhantomData<*mut T>,
}

// Similar to an make_place for IntermediateBox
fn alloc_fastbox<T>(length: usize) -> FastBox<T> {
    let typesize = mem::size_of::<T>();
    let size = length * typesize;
    let align = mem::align_of::<T>();

    let p = if typesize == 0 || length == 0 {
        heap::EMPTY as *mut T
    } else {
        let p = unsafe {
            heap::allocate(size, align) as *mut T
        };
        if p.is_null() {
            panic!("FastBox make_place allocation failure.");
        }
        p
    };

    FastBox { ptr: p, length: length, typesize: typesize, size: size, align: align }
}

impl<T: Sized> Drop for FastBox<T> {
    fn drop(&mut self) {
        if self.typesize > 0 && self.length > 0 {
            unsafe {
                heap::deallocate(self.ptr as *mut u8, self.size, self.align)
            }
        }
    }
}


impl<T> Deref for FastBox<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.length)
        }
    }
}

impl<T> DerefMut for FastBox<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr, self.length)
        }
    }
}

impl<T> ops::Mul<T> for FastBox<T> where T: Float {
    type Output = FastBox<T>;
    fn mul(self, f: T) -> FastBox<T> {
        let mut fb: FastBox<T> = alloc_fastbox::<T>(self.length);
        for (xout,xin) in &mut fb.iter_mut().zip(self.iter()) {
            *xout = f*(*xin);
        }
        fb
    }
}

impl<T> FastBox<T> where T: Float {
    pub fn sin(&self) -> FastBox<T> {
        let mut fb: FastBox<T> = alloc_fastbox::<T>(self.length);
        for (xout,xin) in &mut fb.iter_mut().zip(self.iter()) {
            *xout = (*xin).sin();
        }
        fb
    }
}

impl<T> FastBox<T> where T: Float {
    pub fn sinc(&self) -> FastBox<T> {
        let mut fb: FastBox<T> = alloc_fastbox::<T>(self.length);
        for (xout,xin) in &mut fb.iter_mut().zip(self.iter()) {
            if *xin != T::zero() {
                *xout = (*xin).sin()/(*xin);
            } else {
                *xout = T::one()
            }
        }
        fb
    }
}

pub fn linspace_fastbox<'a, T: 'a>(start: T, stop: T, len: usize) -> FastBox<T>
    where T: Float {

    let (one, zero, dx) = get_values_as_type_t::<T>(start, stop, len);

    let fb: FastBox<T> = alloc_fastbox::<T>(len);
    let ptr = fb.ptr as *mut T;

    unsafe {
        let mut c = zero;

        for ii in 0..len {
            let x = ptr.offset((ii as isize));
            *x = start + c*dx;
            c = c + one;
        }

        return fb
    }
}


pub fn linspace_boxed_slice<'a, T: 'a>(start: T, stop: T, len: usize) -> Box<&'a mut [T]>
    where T: Float {

    let (one, zero, dx) = get_values_as_type_t::<T>(start, stop, len);

    let size = len * mem::size_of::<T>();

    unsafe {
        let ptr = heap::allocate(size, align_of::<T>()) as *mut T;

        let mut c = zero;

        for ii in 0..len {
            let x = ptr.offset((ii as isize));
            *x = start + c*dx;
            c = c + one;
        }

        let sl = slice::from_raw_parts_mut(ptr, len);
        return Box::new(sl);
    }
}
