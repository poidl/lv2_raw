#[cfg(test)]
use utils;
#[test]
fn mytest()
{
    let ar1 = utils::linspace_box(1f64, 5f64, 6);
    for x in ar1.iter() {
        println!("x: {}", x);
    }
    let ptr: *const f64 = &(ar1[0]);
    unsafe {
        for i in 0..5 {
            println!("x: {}", *ptr.offset(i))
        }
    }

    // let y = make_array_unsafe::<f64>(1f64, 5f64, 5);
    println!("ar1: {:?}",ar1);
    // println!("y: {:?}",y);
//transform(data: &mut [f64], stride: u64, n: u64)
}
