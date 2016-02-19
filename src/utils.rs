extern crate rgsl;

use std::f64;

pub fn linspace(slice: &mut [f64], start: f64, stop: f64) -> &mut [f64] {

    let dx = (stop-start)/((slice.len()-1) as f64);
    let mut c :i32 =0;
    unsafe {
        for ii in 0..slice.len() {
            let x = slice.get_unchecked_mut(ii);
            *x = start + (c as f64)*dx;
            c = c + 1;
        }
        return slice
    }
}

pub trait Mult<RHS: ?Sized> {
    fn mult(self: &mut Self, rhs: &RHS);
}

impl Mult<[f64]> for [f64]  {
    fn mult(self: &mut Self, rhs: &Self) {
        for ii in 0..self.len() {
            unsafe {
                let x = self.get_unchecked_mut(ii);
                let y = rhs.get_unchecked(ii);
                *x=*x*(*y);
            }
        }
    }
}


impl Mult<f64> for [f64] {
    fn mult(self: &mut Self, c: &f64) {
        for ii in 0..self.len() {
            unsafe {
                let x = self.get_unchecked_mut(ii);
                *x=*x*c;
            }
        }
    }
}

pub trait Mycopy {
    fn mycopy(self: &mut Self, rhs: &Self);
}

impl  Mycopy for [f64] {
    fn mycopy(self: &mut Self, rhs: &Self) {
        for ii in 0..self.len() {
            unsafe {
                let x = self.get_unchecked_mut(ii);
                let y = rhs.get_unchecked(ii);
                *x=*y;
            }
        }
    }
}

// pub fn mult_inplace(c: f64, slice: &mut [f64]) -> &mut [f64] {
//     for ii in 0..slice.len() {
//         unsafe {
//             let x = slice.get_unchecked_mut(ii);
//             *x=*x*c;
//         }
//     }
//     slice
// }

fn sinc_element(x: &mut f64) -> &mut f64 {
    if *x != 0f64 {
        *x = (*x*f64::consts::PI).sin()/(*x*f64::consts::PI);
    } else {
        *x = 1f64;
    }
    x
}

pub trait Sinc {
    fn sinc(self: &mut Self);
}

impl Sinc for [f64] {
    fn sinc(self: &mut Self) {
        unsafe {
            for ii in 0..self.len() {
                let x = self.get_unchecked_mut(ii);
                sinc_element(x);
            }
        }
    }
}

fn kaiser_element(n: u32, alpha: f64, len: u32) -> f64 {
    let pi = f64::consts::PI;
    let nf = n as f64;
    let lenf = len as f64;
    let mut tmp= 2f64*nf/(lenf-1f64)-1f64;
    tmp = pi*alpha* ( 1f64- tmp.powf(2f64) ).sqrt();
    rgsl::bessel::I0(tmp) / rgsl::bessel::I0( pi*alpha )
}

pub trait Kaiser {
    fn kaiser(self: &mut Self, alpha: f64);
}

impl Kaiser for [f64] {
    fn kaiser(self: &mut Self, alpha: f64) {
        for ii in 0..self.len() {
            let kaiser = kaiser_element(ii as u32,alpha, self.len() as u32);
            unsafe {
                let x = self.get_unchecked_mut(ii);
                *x=*x*kaiser;
            }
        }
    }
}

pub trait Cumsum {
    fn cumsum(self: &mut Self);
}

impl Cumsum for [f64] {
    fn cumsum(self: &mut Self) {
        unsafe {
            let ptr = self.as_mut_ptr();
            for ii in 1..self.len() {
                let xm1 = ptr.offset(ii as isize -1);
                let  x = ptr.offset(ii as isize);
                *x=*x+*xm1;
            }
        }
    }
}

pub fn blit_4T() -> Box<[f64]> {
    // Bandlimited impulse segment for sawtooth with BLIT (bandlimited impulse train) approach. References:
    // Stilson, T. and Smith, J., 1996: Alias-free digital synthesis of classic analog waveforms. Proc. International Computer Music Conference
    // Frei, B.: Digital sound generation. Institute for Computer Music and Sound Technology (ICST) Zurich University of the Arts.
    // See Frei's Fig. 17.
    // see also examples/blit.rs
    let nt: usize = 4;
    let nppt: usize = 2700;
    // N needs to be a constant if we want to stay in the heap memory (2016/01/22)
    // Can't use variables nt and nppt
    const N: usize = 4*(2700-1)+1; // Formula: nt*(nppt-1)+1 as usize; nt is even, therefore N must be uneven
    if nt%2 != 0 {
        panic!("nt is not even");
    }
    if N != nt*(nppt-1)+1 {
        panic!("inconsistent variables");
    }

    let fs = 48000f64; // sampling frequency 1/T.
    let fc = 15000f64; // cutoff frequency 1/Tc.

    let pi = f64::consts::PI;
    let alpha = 8.3f64/pi; // alpha for Kaiser window. Note that beta = pi*alpha.
    let alpha_apo = 0.5f64/pi; // apodization
    let apof = 0.5f64;

    let nipt = (nppt-1) as f64; // number of Tl per T

    // as time axis we use units of Tl instead of T for the calculation, to keep
    // the formula for the Fourier trafo clean (linear axis scaling). Later
    // in the plot we will drop the factor nipt and display in units of T.
    let nth = 0.5f64*(nt as f64)*nipt;

    let mut t = vec![0f64;N];
    linspace(&mut t,-nth, nth);

    // The first zero crossing of the impulse determines the cutoff frequency fc=(1/Tc).
    // The normalized sinc function sinc(T) has the first zero crossing at 1*T,
    // corresponding to a cutoff at 0.5(1/T)=0.5*fs, which differs by a factor of
    // c=2*fc/fs from fc.  To get the cutoff at fc, we use c*sinc(c*T) (see doc for
    // the effect of linear axis scaling on the Fourier transform). Also include
    // a factor 1/nipt to transform T into Tl

    let c =  2f64*fc/(fs*nipt);

    // impulse h
    t.mult(&c); let mut ct=t;
    ct.sinc(); let mut sinc_ct= ct;
    sinc_ct.mult(&c); let mut h = sinc_ct;

    // apply Kaiser window
    h.kaiser(alpha); let mut hk = h;

    // apodization (scale the maximum to c afterwards)
    let mut kaiser_apo = vec![1f64;N];
    kaiser_apo.kaiser(alpha_apo);

    for i in 0..kaiser_apo.len() {
        kaiser_apo[i]=1f64-apof*kaiser_apo[i];
    }
    hk.mult(&kaiser_apo[..]);

    let mut tmp =  vec![0f64;N];
    tmp.mycopy(&hk);
    tmp.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let max=tmp[N-1];
    hk.mult(&(c*(1f64/max)));

    // integrate and scale hk(0) (i.e. middle) to 1
    hk.cumsum(); let mut cs = hk;
    let middle = cs[cs.len()/2+1];
    cs.mult(&(1f64/middle));
    // flip cs(t>0) around t axis
    for ii in cs.len()/2+2 .. N {
        cs[ii]=-2f64+cs[ii];
    }
    cs.into_boxed_slice()
}
