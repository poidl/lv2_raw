#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

#![feature(test)]

extern crate test; // for bench
extern crate num;

use num::NumCast;
use num::Float;
use std::mem;
use test::Bencher;



pub fn make_arr_unsafe<'a, T: 'a>(start: T, stop: T, num: usize)
// -> Box<[T]>
{
    let size = mem::size_of::<T>();

}

fn linspace_vec<'a, T: 'a>(start: T, stop: T, num: usize) ->
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

fn linspace_box<'a, T: 'a>(start: T, stop: T, num: usize) ->
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




#[bench]
fn bench1(b: &mut Bencher) {
    b.iter(|| linspace_box(1f64, 5f64, 1000000));
}

#[bench]
fn bench2(b: &mut Bencher) {
    b.iter(|| linspace_vec(1f64, 5f64, 1000000));
}
