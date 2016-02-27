#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// #![feature(alloc)]
//#![feature(heap_api)]
// #![feature(unique)]


//mod tests;
pub mod utils;
mod lv2;
//mod heapslice;

extern crate libc;
use std::ptr;
use std::mem;
use std::str;
use std::ffi::CString;
use std::ffi::CStr;
use utils::*;

macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

enum PortIndex {
    MidiIn = 0,
    AudioOut = 1
}

// Unnecessary? See comment in connect_port().
impl PortIndex {
    fn from_int(x: u32) -> PortIndex {
        match x {
            0 => PortIndex::MidiIn,
            1 => PortIndex::AudioOut,
            _ => panic!("Not a valid PortIndex: {}", x)
        }
    }
}

#[repr(C)]
struct Synth {
    map: *const lv2::Lv2uridMap,
    in_port: *const lv2::LV2_Atom_Sequence,
    output: *mut f32,
    uris: Synthuris,
    fs: f64,
    currentfreq: f64,
    currentmidivel: u8,
    noteison: bool,
    makesilence: bool,
    osc: Osc,
    oscST: OscST
}

impl lv2::LV2Descriptor {
    pub extern fn instantiate( _descriptor: *const lv2::LV2Descriptor , fs: f64, bundle_path: *const libc::c_char , features: *const (*const lv2::LV2Feature),) -> lv2::Lv2handle {
        unsafe{
        let ptr = libc::calloc(1,mem::size_of::<Synth>() as libc::size_t);
        if ptr.is_null() {
            panic!("failed to allocate memory");
        }

        let mut map = (*(ptr  as *mut Synth)).map;

        let uridmapstr = "http://lv2plug.in/ns/ext/urid#map";
        let mut x: isize = 0;
        let mut done = false;
        while !done {

                let fptr: *const lv2::LV2Feature = *features.offset(x);
                if fptr.is_null() {
                    // host doesn't provide feature
                    libc::free(ptr as *mut libc::c_void);
                    println!("Missing feature \"{}\"", uridmapstr);
                    return std::ptr::null_mut();
                }
                let uriptr = (*fptr).uri;
                let buf = CStr::from_ptr(uriptr).to_bytes();
                let s: &str = str::from_utf8(buf).unwrap();
                println!("uri: {}", s);
                if s == uridmapstr {

                    map = (*fptr).data;
                    done=true;
                    println!{" -> obtained urid#map from host"}
                }

            x = x+1;
        }

        let uris_addr = &mut (*(ptr  as *mut Synth)).uris;
        map_synth_uris(map, uris_addr);

        (*(ptr  as *mut Synth)).fs = fs;
        (*(ptr  as *mut Synth)).noteison = false;
        (*(ptr  as *mut Synth)).makesilence = false;
        (*(ptr  as *mut Synth)).osc = Osc { phase: 0, dphase: 0 };
        (*(ptr  as *mut Synth)).osc.set_dphase(440.0,(*(ptr  as *mut Synth)).fs);
            println!("self.dphase: {}",(*(ptr  as *mut Synth)).osc.dphase);

        (*(ptr  as *mut Synth)).oscST = OscST {
            N: 0u32,
            A: 0i32,
            fnn: 0u32,
            B: 0i32,
            alpha: 0u32,
            M: 0u32,
            i: 0i32,
            f: (*Box::into_raw(utils::blit_4T())).as_ptr(), // TODO: this must be cleaned up? See https://doc.rust-lang.org/std/primitive.pointer.html
            C: 0f64,
            D: 0f64,
            fs: 0f64,
            f0: 0f64,
            fac_i: 0f64,
            fac_alpha: 0f64,
            fac_fn: 0f64,
            absA: 0i32
        };

        ptr
        }
    }

    pub extern fn connect_port(handle: lv2::Lv2handle, port: u32, data: *mut libc::c_void) {
        let synth: *mut Synth = handle as *mut Synth;
        // simpler to use PortIndex instead of u32 for port, but that doesn't correspond to C?
        match PortIndex::from_int(port) {
            // data may be NULL pointer -> don't dereference!
        // match port {
            PortIndex::MidiIn => unsafe{ (*synth).in_port = data  as *const lv2::LV2_Atom_Sequence},
            PortIndex::AudioOut => unsafe{ (*synth).output = data as *mut f32 },
        }

    }
    pub extern fn activate(_instance: lv2::Lv2handle) {}

    pub extern fn run(instance: lv2::Lv2handle, n_samples: u32) {
        unsafe{
            let synth = instance as *mut Synth;
            let uris = &mut (*synth).uris;
            let seq = (*synth).in_port;
            let output = (*synth).output;
            // pointer to 1st event body
            let mut ev: *const lv2::Lv2AtomEvent  = lv2::lv2_atom_sequence_begin(&(*seq).body);
            // loop through event sequence
            while !lv2::lv2_atom_sequence_is_end(&(*seq).body, (*seq).atom.size, ev) {
                // check if event is midi
                if (*ev).body.mytype == (*uris).midi_event {

                    // pointer to midi event data
                    let msg: *const u8 = ev.offset(1) as *const u8;
                    // frameindex of eventstart. In jalv this is relative to currently processed buffer chunk of length n_samples
                    let istart = (*ev).time_in_frames as u32;

                    match lv2::lv2_midi_message_type(msg) {

                        // note on event
                        lv2::Lv2MidiMessageType::Lv2MidiMsgNoteOn => {
                            (*synth).noteison = true;
                            let f0 = f0_from_midimsg(msg);
                            (*synth).currentfreq = f0;
                            (*synth).currentmidivel = *msg.offset(2);
                            let coef = 1.0 as f32;

                            (*synth).osc.reset();
                            (*synth).osc.set_dphase(f0,(*synth).fs);

                            // TODO don't set fs here
                            (*synth).oscST.reset((*synth).fs);
                            (*synth).oscST.set_f0fn(f0);

                            for i in istart-1..n_samples {
                                // let amp = (*synth).osc.get() as f32;
                                let amp = (*synth).oscST.get() as f32;
                                *output.offset(i as isize) = amp;
                            }
                        }

                        // note off event
                        lv2::Lv2MidiMessageType::Lv2MidiMsgNoteOff => {
                            (*synth).noteison = false;
                            (*synth).makesilence = true;
                            for i in istart-1..n_samples {
                                let amp = 0.0 as f32;
                                *output.offset(i as isize) = amp as f32;
                            }
                        }

                        _ => {
                            println!("DON'T UNDERSTAND MESSAGE")
                        }

                    }
                }
                ev = lv2::lv2_atom_sequence_next(ev);
            }

            if (*synth).noteison {
                let coef = 1.0 as f32;
                let f0 = (*synth).currentfreq;

                for i in 0..n_samples {
                    // let amp = (*synth).osc.get();
                    let amp = (*synth).oscST.get();
                    *output.offset(i as isize) = (amp as f32) * coef;
                }

            } else if (*synth).makesilence {
                (*synth).makesilence = false;
                for i in 0..n_samples {
                    let amp = 0.0;
                    *output.offset(i as isize) = amp as f32;
                }
            }

        }

    }

    pub extern fn deactivate(_instance: lv2::Lv2handle) {}
    pub extern fn cleanup(instance: lv2::Lv2handle) {

        unsafe{
            //ptr::read(instance as *mut Amp); // no need for this?
            libc::free(instance  as lv2::Lv2handle)
        }
    }
    pub extern fn extension_data(_uri: *const u8)-> (*const libc::c_void) {
                            ptr::null()
    }
}

static S: &'static [u8] = b"http://example.org/yassy\0";
static mut desc: lv2::LV2Descriptor = lv2::LV2Descriptor {
    uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
    instantiate: lv2::LV2Descriptor::instantiate,
    connect_port: lv2::LV2Descriptor::connect_port,
    activate: lv2::LV2Descriptor::activate,
    run: lv2::LV2Descriptor::run,
    deactivate: lv2::LV2Descriptor::deactivate,
    cleanup: lv2::LV2Descriptor::cleanup,
    extension_data: lv2::LV2Descriptor::extension_data
};

#[no_mangle]
pub extern fn lv2_descriptor(index:i32) -> *const lv2::LV2Descriptor {
    if index != 0 {
        return ptr::null();
    } else {
        // credits to ker on stackoverflow: http://stackoverflow.com/questions/31334356/static-struct-with-c-strings-for-lv2-plugin (duplicate) or http://stackoverflow.com/questions/25880043/creating-a-static-c-struct-containing-strings
        let ptr = S.as_ptr() as *const libc::c_char;
        unsafe {
        desc.uri = ptr;
        return &desc as *const lv2::LV2Descriptor
        }
    }
}

struct Synthuris {
    midi_event: lv2::Lv2urid
}

fn map_synth_uris(map: *const lv2::Lv2uridMap, uris: &mut Synthuris) {
    let s = "http://lv2plug.in/ns/ext/midi#MidiEvent";
    let cstr = CString::new(s).unwrap();
    let lv2_midi_midi_event = cstr.as_ptr();
    //mem::forget(cstr);
    unsafe{
        uris.midi_event = ((*map).map)((*map).handle, lv2_midi_midi_event);
    }
}

fn f0_from_midimsg(msg: *const u8) -> f64 {
    // A3 has midi number 56
    // Frequencies are calculated with the formula
    // f0 = {[(2)^1/12]^n} * 220 Hz,
    // where n is the number of half steps from A3
    unsafe{
        let i = *msg.offset(1);
        let f0 = (2.0f64.powf((((i as i8)-57) as f64)/12.0))*220.0;
        // println!("FREQ: {}", freq);
        return f0
    }
}

fn amp(isample: u32, f0: f64, fs: f64) -> f64 {
    let lam = fs/f0;
    // println!("fs: {}", fs);
    return (2.0*std::f64::consts::PI*((isample as f64)/lam)).sin()
}

struct Osc {
    phase: u32,
    dphase: u32
}

impl Osc {
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

pub struct OscST {
    // We translate the fundamental frequency f0 from units 1/t to a fraction "fn" of a wavetable with 2N lattice points. fn corresponds to the number of points which are skipped when reading the wavetable and can be interpreted as a phase increment. The 2N lattice points represent the interval [-pi,pi). The max. resolved freq. f0=fs/2, i.e. we want that fn(fs/2)=N and fn(0)=0. The function is linear, hence fn(f0)=2N*f0/fs. If a sined integer of k bits is used as phase accumulator, the 2N interval translates to [-2^(k-1),2^(k-1)). Note the square bracket (paranthesis) on the left (right). For k=2, the values range from -2 to 1.
    pub N: u32,
    pub A: i32, // phase. Wavetable size is 2N. start at zero, wrap at N from 1 to
    // -1
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
