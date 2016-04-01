pub struct OscBasic {
    phase: u32,
    dphase: u32
}

impl OscBasic {
    fn reset(& mut self) {
        self.phase =  0
    }
    fn set_dphase(&mut self, f0: f64, fs: f64) {
		// Phase increment of the phase accumulator. (f0/fs) is the
        // fraction of period per sample. This is multiplied by 2^32, so
        // each frequency is equivalent to a fraction of the "maximum
        // phase increment" 2^32, which corresponds to  f0 = fs.
		// (2^32)/16=268435456
        self.dphase =  ((f0/fs)*4294967296.0) as u32;
        //println!("bla: {}",f0*(0xFFFFFFFF as u32))
    }
    fn step(&mut self){
        //let x = Wrapping(self.phase);
        //let y = Wrapping(self.dphase);
        //self.phase = (x+y).0;
        // wrapping_add: allows intentional overflow
        self.phase = self.phase.wrapping_add(self.dphase);
    }
    fn get(&mut self) -> f32 {
        self.step();
        let phi: f32 = (self.phase as f64/2147483648.0 -1f64) as f32;
        return phi
    }
}
