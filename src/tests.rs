#[cfg(test)]

use utils;
use utils::Castd;

#[test]

fn mytest()
{
    // let v = utils::linspace_vec(1f64, 5f64, 6);
    //
    // for x in v.iter() {
    //     println!("x: {}", x);
    // }
    // let ar1 = utils::linspace_slice(1f64, 5f64, 6);
    // for x in ar1.iter() {
    //     println!("x: {}", x);
    // }
    // let ptr: *const f64 = &(ar1[0]);
    // unsafe {
    //     for i in 0..6 {
    //         println!("x: {}", *ptr.offset(i))
    //     }
    // }
    //
    // let ptr = utils::linspace_ptr(1f64, 5f64, 6);
    // // for x in ar1.iter() {
    //     // println!("x: {}", x);
    // // }
    // unsafe {
    //     for i in 0..6 {
    //         println!("x: {}", *ptr.offset(i))
    //     }
    // }
    // let ptr = utils::linspace_ptr(1f64, 5f64, 6);
    // // for x in ar1.iter() {
    // //     println!("x: {}", x);
    // // }
    // unsafe {
    //     for i in 0..6 {
    //         println!("x: {}", *ptr.offset(i))
    //     }
    // }
    let x = utils::linspace_heapslice(1f64, 5f64, 6);
    // let x = utils::linspace_vec(1f64, 5f64, 6);
    // let mut y=x.clone();
    // y[3]=1000f64;
    //
    // let n=10i32;
    // let z=n.d()*x.clone();
    //
    // for xi in x.iter() {
    //     println!("xi: {}", xi);
    // }
    // for yi in y.iter() {
    //     println!("yi: {}", yi);
    // }
    // for zi in z.iter() {
    //     println!("yi: {}", zi);
    // }
    // let heapslice = heapslice*10;
    // for x in heapslice.iter() {
    //     println!("x: {}", x);
    // }
    // for ii in 0..heapslice.len() {
    //     unsafe {
    //         println!("x: {}", heapslice.get_unchecked(ii));
    //         println!("x: {}", heapslice[ii]);
    //     }
    // }
    // let ptr = heapslice.ptr;
    // unsafe {
    //     for i in 0..heapslice.length as isize {
    //         println!("x: {}", *ptr.offset(i))
    //     }
    // }

    // let boxed_slice = utils::linspace_boxed_slice(1f64, 5f64, 6);
    // for x in boxed_slice.iter() {
    //     println!("x: {}", x);
    // }

//
//     // let y = make_array_unsafe::<f64>(1f64, 5f64, 5);
//     println!("ar1: {:?}",ar1);
//     // println!("y: {:?}",y);
// //transform(data: &mut [f64], stride: u64, n: u64)
    // utils::make_arr_unsafe::<f64>(5) ;

}
