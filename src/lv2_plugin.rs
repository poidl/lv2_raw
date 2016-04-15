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
    fn connect_port(&self, u32, *mut libc::c_void);
    fn midievent(&mut self, msg: &u8) ;
    fn set_fs(&mut self, f64);
    fn get_amp(&mut self) -> f32;
}

pub struct Synthuris {
    pub midi_event: lv2::Lv2urid
}

#[repr(C)]
pub struct Lv2SynthPlugin {
    pub map: *const lv2::Lv2uridMap,
    // pub portidx: PortIndex,
    pub in_port: *const lv2::LV2_Atom_Sequence,
    pub output: *mut f32,
    pub uris: Synthuris,
    pub synth: synth::Synth,
    fs: f64
}

impl isLv2SynthPlugin for Lv2SynthPlugin {
    fn connect_port(&self, port: u32, data: *mut libc::c_void) {
        match port {
            0 => unsafe{self.in_port = data  as *const lv2::LV2_Atom_Sequence},
            1 => unsafe{self.output = data as *mut f32 },
            2 => unsafe{self.gain = data as *mut f32 },
            _ => panic!("Not a valid PortIndex: {}", port)
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
