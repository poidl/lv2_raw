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


pub fn linspace_vec<'a, T: 'a>(start: T, stop: T, num: usize) ->
Vec<T>
//Box<[T]>
    where T: Float {

    let zero: T = num::cast(0).unwrap();
    let num_t: T = num::cast(num).unwrap();
    let one: T = num::cast(1).unwrap();
    let diff = stop - start;
    let dx = diff/(num_t-one);

    // let mut bx  = vec![zero; num].into_boxed_slice();
    let mut bx = vec![zero; num];
    let mut c = zero;

    for x in bx.iter_mut() {
        *x = start + c*dx;
        c = c + one;
    }

    return bx
}

pub fn linspace_vec2box<'a, T: 'a>(start: T, stop: T, num: usize) ->
// Vec<T>
Box<[T]>
    where T: Float {

    let zero: T = num::cast(0).unwrap();
    let num_t: T = num::cast(num).unwrap();
    let one: T = num::cast(1).unwrap();
    let diff = stop - start;
    let dx = diff/(num_t-one);

    let mut bx  = vec![zero; num].into_boxed_slice();
    // let mut bx = vec![zero; num];
    let mut c = zero;

    for x in bx.iter_mut() {
        *x = start + c*dx;
        c = c + one;
    }

    return bx
}

pub fn make_arr_unsafe<'a, T>(len: usize) -> &'a mut [T] {

    let size = len * mem::size_of::<T>();
    // println!("size: {}", size);
    unsafe {
        let ptr = heap::allocate(size, align_of::<T>());
        intrinsics::volatile_set_memory(ptr, 0, size);
        let slice = slice::from_raw_parts_mut(ptr as *mut T, len);
        return slice;
    }
}

pub fn linspace_slice<'a, T: 'a>(start: T, stop: T, num: usize) ->
// Vec<T>
&'a [T]
    where T: Float {

    let zero: T = num::cast(0).unwrap();
    let num_t: T = num::cast(num).unwrap();
    let one: T = num::cast(1).unwrap();
    let diff = stop - start;
    let dx = diff/(num_t-one);

    // let bx = make_arr_unsafe::<T>(num) ;
    ////////////
    let size = num * mem::size_of::<T>();
    // println!("size: {}", size);
    unsafe {
        let ptr = heap::allocate(size, align_of::<T>());
        //intrinsics::volatile_set_memory(ptr, 0, size);
        let bx = slice::from_raw_parts_mut(ptr as *mut T, num);

        let mut c = zero;

        // for ii in 0..num {
        //     let x = bx.get_unchecked_mut(ii);
        //     *x = start + c*dx;
        //     c = c + one;
        // }
        for x in bx.iter_mut() {
            *x = start + c*dx;
            c = c + one;
        }

        return bx
    }
}

pub fn linspace_ptr<'a, T: 'a>(start: T, stop: T, num: usize) ->
// Vec<T>
*mut T
    where T: Float {

    let zero: T = num::cast(0).unwrap();
    let num_t: T = num::cast(num).unwrap();
    let one: T = num::cast(1).unwrap();
    let diff = stop - start;
    let dx = diff/(num_t-one);

    // let bx = make_arr_unsafe::<T>(num) ;
    ////////////
    let size = num * mem::size_of::<T>();
    // println!("size: {}", size);
    unsafe {
        let ptr = heap::allocate(size, align_of::<T>()) as *mut T;
        //intrinsics::volatile_set_memory(ptr, 0, size);
        //let bx = slice::from_raw_parts_mut(ptr as *mut T, num);

        let mut c = zero;

        for ii in 0..num {
            let x = ptr.offset((ii as isize));
            *x = start + c*dx;
            c = c + one;
        }
        // for x in bx.iter_mut() {
        //     *x = start + c*dx;
        //     c = c + one;
        // }

        return ptr as *mut T
    }
}
