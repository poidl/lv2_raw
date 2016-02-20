extern crate  yassy;
use yassy::*;

#[test]
fn mytest() {
 let mut osc = yassy::OscST { N: 0, phase: 0, dphase: 0, alpha: 0f64};
 osc.reset();
 osc.set_dphase(4000f64, 41000f64);
 osc.set_alpha(4000f64, 41000f64);
 for ii in 0..300 {
     let amp = osc.get();
     println!("osc.get(): {}", amp);
     if (amp.abs() as f64) < (osc.alpha)  {
         println!("apply segment ************");
     }
 }
}
