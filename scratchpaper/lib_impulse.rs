#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

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

const N : usize = 1;
// const NN : usize= 41000-N;
const NN : usize= 82;

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
    rate: f64,
    currentfreq: f64,
    currentmidivel: u8,
    noteison: bool,
    makesilence: bool,
    osc: Osc,
    impulse: Impulse
}

impl lv2::LV2Descriptor {
    pub extern fn instantiate( _descriptor: *const lv2::LV2Descriptor , rate: f64, bundle_path: *const libc::c_char , features: *const (*const lv2::LV2Feature),) -> lv2::Lv2handle {
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

        (*(ptr  as *mut Synth)).rate = rate;
        (*(ptr  as *mut Synth)).noteison = false;
        (*(ptr  as *mut Synth)).makesilence = false;
        (*(ptr  as *mut Synth)).osc = Osc { phase: 0, dphase: 0 };
        (*(ptr  as *mut Synth)).osc.set_dphase(440.0,(*(ptr  as *mut Synth)).rate);
            println!("self.dphase: {}",(*(ptr  as *mut Synth)).osc.dphase);

        // define impulse
        let im = &mut (*(ptr  as *mut Synth)).impulse;
        for i in im.buf[..N].iter_mut() {
            *i = 1f64;
        }
        // let pi = std::f64::consts::PI;
        // let alpha = 9f64/pi; // alpha for Kaiser window.
        // im.buf[..N].kaiser(alpha);
        // // for i in im.buf.iter() {
        // //     println!("buf: {}", i);
        // // }
        for i in im.buf[NN..].iter_mut() {
            *i = 0f64;
        }
        im.offs = 0;
        return ptr;

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
                            let freq = freq_from_midimsg(msg);
                            (*synth).currentfreq = freq;
                            (*synth).currentmidivel = *msg.offset(2);
                            let coef = 1.0 as f32;

                            // (*synth).osc.reset();
                            // (*synth).osc.set_dphase(freq,(*synth).rate);
                            //
                            // for i in istart-1..n_samples {
                            //     let amp = (*synth).osc.get();
                            //     *output.offset(i as isize) = amp;
                            // }
                            (*synth).impulse.reset();

                            for i in istart-1..n_samples {
                                let amp = (*synth).impulse.get();
                                // println!("amp: {}", amp);
                                *output.offset(i as isize) = amp as f32;
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
                let freq = (*synth).currentfreq;

                // for i in 0..n_samples {
                //     let amp = (*synth).osc.get();
                //     *output.offset(i as isize) = (amp as f32) * coef;
                // }
                for i in 0..n_samples {
                    let amp = (*synth).impulse.get();
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

fn freq_from_midimsg(msg: *const u8) -> f64 {
    // A3 has midi number 56
    // Frequencies are calculated with the formula
    // freq = {[(2)^1/12]^n} * 220 Hz,
    // where n is the number of half steps from A3
    unsafe{
        let i = *msg.offset(1);
        let freq = (2.0f64.powf((((i as i8)-57) as f64)/12.0))*220.0;
        // println!("FREQ: {}", freq);
        return freq
    }
}

fn amp(isample: u32, freq: f64, rate: f64) -> f64 {
    let lam = rate/freq;
    // println!("rate: {}", rate);
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
    fn set_dphase(&mut self, freq: f64, rate: f64) {
		// Phase increment of the phase accumulator. (freq/rate) is the
        // fraction of period per sample. This is multiplied by 2^32, so
        // each frequency is equivalent to a fraction of the "maximum
        // phase increment" 2^32, which corresponds to  freq = rate.
		// (2^32)/16=268435456
        self.dphase =  ((freq/rate)*4294967296.0) as u32;
        //println!("bla: {}",freq*(0xFFFFFFFF as u32))
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
        let phi: f32 = (self.phase as f64/4294967296.0) as f32;
        return phi
    }
}

struct Impulse {
    // N+NN samples for one period, consisting of N samples during which we play, and NN samples of silence
    buf: [f64;N+NN], // buffer for waveform
    offs: u32 // wave offset
}

impl Impulse {
    fn reset(& mut self) {
        self.offs =  0
    }
    fn step(&mut self){
        self.offs = (self.offs+1) % self.buf.len() as u32;
    }
    fn get(&mut self) -> f64 {
        let hoit = self.buf[self.offs as usize];
        self.step();
        return hoit
    }
}
