#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

extern crate libc;

use std::ffi::CString;
use synth;

use midi;
use midi::*;
use synth::*;

pub struct SynthPlugin {
    pub midi_in: *const u8,
    pub audio_out: *mut f32,
    pub synth: synth::Synth,
    fs: f64
}

impl  SynthPlugin {
    pub fn new() -> SynthPlugin {
        SynthPlugin {
            midi_in: &0u8,
            audio_out: &mut 0f32,
            synth: synth::Synth::new(),
            fs: 0f64
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
    pub fn midievent(&mut self, msg: &u8) {
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
    pub fn set_fs(&mut self, fs: f64) {
        self.synth.set_fs(fs);
    }
    pub fn get_amp(&mut self) -> f32 {
        self.synth.get_amp()
    }
}
