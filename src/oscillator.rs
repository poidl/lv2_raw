
pub trait Oscillator {
    fn set_fs(&mut self, f64);
    fn set_f0(&mut self, f32);
    fn reset_phase(&mut self);
    fn get_amp(&mut self) -> f32;
}

impl Oscillator for OscBasic {
    fn set_fs(&mut self, fs: f64) {
        self.fs = fs;
    }
    fn set_f0(&mut self, f0: f32) {
		// Phase increment of the phase accumulator. (f0/fs) is the
        // fraction of period per sample. This is multiplied by 2^32, so
        // each frequency is equivalent to a fraction of the "maximum
        // phase increment" 2^32, which corresponds to  f0 = fs.
		// (2^32)/16=268435456
        self.dphase =  ((f0/self.fs as f32)*4294967296.0) as u32;
    }
    fn reset_phase(& mut self) {
        self.phase =  0
    }
    fn get_amp(&mut self) -> f32 {
        self.step();
        let phi: f32 = (self.phase as f64/2147483648.0 -1f64) as f32;
        return phi
    }
}

pub struct OscBasic {
    fs: f64,
    pub phase: u32,
    pub dphase: u32
}

impl OscBasic {
    fn step(&mut self){
        self.phase = self.phase.wrapping_add(self.dphase);
    }
}

impl OscBasic {
    pub fn new() -> OscBasic {
        OscBasic {
            fs: 0f64,
            phase: 0u32,
            dphase: 0u32
        }
    }
}

pub struct OscST {
    // We translate the fundamental frequency f0 from units 1/t to a fraction "fn" of a wavetable with 2N lattice points. fn corresponds to the number of points which are skipped when reading the wavetable and can be interpreted as a phase increment. The 2N lattice points represent the interval [-pi,pi). The max. resolved freq. f0=fs/2, i.e. we want that fn(0)=0 and fn(fs/2)=N. The function is linear, hence fn(f0)=2N*f0/fs. If a sined integer of k bits is used as phase accumulator, the 2N interval translates to [-2^(k-1),2^(k-1)). Note the interval is open on the left. For k=2, the values range from -2 to 1.
    pub N: u32,
    pub A: i32, // phase. Wavetable size is 2N. start at zero, wrap at N from 1 to -1
    pub fnn: u32, // phase increment
    pub B: i32, // A, phase shifted by N
    pub alpha: u32,
    pub M: u32, // number of entries in half-segment of integratied bandlimited impulse
    pub i: i32,
    pub f: *const f64,
    pub C: f64,
    pub D: f64,
    pub fs: f64, // sample rate
    pub f0: f64, // fundamental frequency
    pub fac_i: f64, // avoid unnecessary runtime multiplication
    pub fac_alpha: f64,
    pub fac_fn: f64,
    pub absA: i32
}

impl OscST {
    pub fn reset(& mut self, fs: f64) {
        self.N = 2u32.pow(31); // follow notation of Frei (p. 3)
        self.M = (2*(2700-1)+1) as u32;
        self.B =  0;
        self.A =  self.B.wrapping_add(self.N as i32);
        self.fs = fs;
        let c = 4 as f64 * self.N as f64;
        self.fac_i = self.M as f64 *fs/c;
        self.fac_alpha = c/fs;
        self.fac_fn = 2f64*self.N as f64/self.fs;
    }
    pub fn set_f0fn(&mut self, f0: f64) {
        self.f0 = f0;
        self.fnn =  (f0*self.fac_fn) as u32;
    }
    pub fn step_AB(&mut self){
        // wrapping_add: allows intentional overflow
        self.B = self.B.wrapping_add(self.fnn as i32);
        self.A = self.B.wrapping_add(self.N as i32);
        // A.abs() will panic/overflow if A=i32::min_value().
        let mask = self.A >> 31u32;
        self.absA = self.A ^ mask; // xor with mask is equivalent to -1*(A+1) for A<0, and a no-op otherwise. http://stackoverflow.com/questions/12041632/how-to-compute-the-integer-absolute-value
    }
    pub fn set_alpha_i(&mut self) {
        self.alpha =  (self.f0*self.fac_alpha) as u32;
        let tmp = (self.A as f64 /self.f0) *self.fac_i;
        self.i = tmp.trunc() as i32;
    }
    pub fn step_C(&mut self) {
        if self.absA < (self.alpha as i32) {
            unsafe {
                self.C = -*self.f.offset(self.M as isize + self.i as isize);
            }
            // println!("apply {}", self.C);
        } else {
            self.C = 0f64;
        }
    }
    pub fn step_D(&mut self) {
        let N = self.N as f64;
        // println!("self.B {}", self.B as f64/ N );
        // println!("self.C {}", self.C);
        // println!("self.i {}", self.i);
        // println!(" ");
        self.D = self.C + self.B as f64/ N
    }
    pub fn get(&mut self) -> f64 {
        self.step_AB();
        self.set_alpha_i();
        self.step_C();
        self.step_D();
        self.D as f64
    }
}
