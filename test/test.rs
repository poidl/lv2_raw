#[cfg(test)]

#[test]
fn mytest() {
 let osc = OscST;
 osc.reset();
 osc.set_dphase(4000f64, 41000f64)
 for ii in 0..100 {
     println!("osc.get(): ", osc.get());
 }
}
