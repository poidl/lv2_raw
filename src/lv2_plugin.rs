#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

extern crate libc;

use std::ffi::CString;
use synth;
use lv2;
use midi;
use midi::*;
use synth::*;

// enum PortIndex {
//     MidiIn = 0,
//     AudioOut = 1,
//     Gain = 2
// }
//
// // Unnecessary? See comment in connect_port().
// impl PortIndex {
//     fn from_int(x: u32) -> PortIndex {
//         match x {
//             0 => PortIndex::MidiIn,
//             1 => PortIndex::AudioOut,
//             2 => PortIndex::Gain,
//             _ => panic!("Not a valid PortIndex: {}", x)
//         }
//     }
// }

pub trait isLv2SynthPlugin: {
    fn connect_port(&mut self, u32, *mut libc::c_void);
    fn midievent(&mut self, msg: &u8) ;
    fn set_fs(&mut self, f64);
    fn get_amp(&mut self) -> f32;
    fn map_params(&mut self, u32, *mut libc::c_void);
}

pub struct Synthuris {
    pub midi_event: lv2::Lv2urid
}

#[repr(C)]
pub struct Lv2SynthPlugin<'a> {
    pub map: *const lv2::Lv2uridMap,
    // pub portidx: PortIndex,
    pub in_port: *const lv2::LV2_Atom_Sequence,
    pub output: *mut f32,
    pub uris: Synthuris,
    pub synth: synth::Synth<'a>,
    fs: f64
}

impl<'a> isLv2SynthPlugin for Lv2SynthPlugin<'a> {
    fn connect_port(&mut self, port: u32, data: *mut libc::c_void) {
        match port {
            0 => unsafe{self.in_port = data  as *const lv2::LV2_Atom_Sequence},
            1 => unsafe{self.output = data as *mut f32 },
            _ => self.map_params(port,data)
        }
    }
    fn map_params(&mut self, port: u32, data: *mut libc::c_void) {
        let nparams = 1;
        let iport = port - 2; //TODO: don't hardcode number of input/output ports
        if (iport <= nparams-1) {
            println!("connecting port: {}", port);
            unsafe{self.synth.params[iport as usize]= &*(data  as *mut f32) };
            // println!("param: {}",  *(self.synth.params[0]));
        } else {
            panic!("Not a valid PortIndex: {}", iport)
        }
    }
    fn midievent(&mut self, msg: &u8) {
        let mm = msg as midi::MidiMessage;
        if mm.noteon() {
            self.synth.noteon(mm.f0(), mm.vel())
        } else if mm.noteoff() {
            self.synth.noteoff();
        } else if mm.cc() {
            println!("ccnr: {}", mm.ccnr());
            println!("ccval: {}", mm.ccval());
        } else {
            println!("Don't understand midi message", );
        }
        // self.synth.midievent(msg);
    }
    fn set_fs(&mut self, fs: f64) {
        self.synth.set_fs(fs);
    }
    fn get_amp(&mut self) -> f32 {
        self.synth.get_amp()
    }
}
