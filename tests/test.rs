extern crate  yassy;
use yassy::*;

#[test]
fn mytest() {
    let mut osc = yassy::OscST {
        N: 0u32,
        A: 0i32,
        fnn: 0u32,
        B: 0i32,
        alpha: 0f64,
        M: 0u32,
        i: 0u32,
        f: [0f64; 2*(2700-1)+1],
        C: 0f64,
        D: 0f64,
        fs: 0f64,
        f0: 0f64,
        fac_i: 0f64,
        fac_alpha: 0f64,
        fac_fn: 0f64
    };

    osc.reset(41000f64);
    osc.set_f0fn(4000f64);
    for ii in 0..10 {
println!("i32 min: {}",i32::min_value());
println!("i32 min: {}",i32::max_value());
        osc.set_alpha_i();
        println!("setasdfad");
        osc.step_C();
        osc.step_D();
        let amp = osc.get();
        println!("A: {}", osc.A as f64/osc.N as f64);
        println!("B: {}", osc.B as f64/osc.N as f64);
        println!("amp: {}", amp);
        if (amp.abs() as f64) < (osc.alpha)  {
            println!("apply segment ************");
        }
        osc.step_AB();
    }
}
