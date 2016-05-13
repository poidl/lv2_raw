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

enum param_name {
    gain,
}

pub trait HasFs { // sample rate
    fn set_fs(&mut self, f64);
}

pub struct SynthPlugin {
    pub midi_in: *const u8,
    pub audio_out: *mut f32,
    pub synth: synth::Synth,
    fs: f64,
    pub params: [*mut f32;1]
}

impl  SynthPlugin {
    pub fn new() -> SynthPlugin {
        SynthPlugin {
            midi_in: &0u8,
            audio_out: &mut 0f32,
            synth: synth::Synth::new(),
            fs: 0f64,
            params: [&mut 0.5f32;1]
        }
    }
    // fn map_params(&mut self, port: u32, data: *mut libc::c_void) {
    //     let nparams = 1;
    //     let iport = port - 2; //TODO: don't hardcode number of input/output ports
    //     if (iport <= nparams-1) {
    //         println!("connecting port: {}", port);
    //         unsafe{self.params[iport as usize]= data  as *mut f32 };
    //         // println!("param: {}",  *(self.synth.params[0]));
    //     } else {
    //         panic!("Not a valid PortIndex: {}", iport)
    //     }
    // }
    pub fn midievent(&mut self, msg: &u8) {
        let mm = msg as midi::MidiMessage;
        if mm.noteon() {
            self.synth.noteon(mm.f0(), mm.vel())
        } else if mm.noteoff() {
            self.synth.noteoff();
        } else if mm.cc() {
            let x = mm.cc_type();
            unsafe {
                match x {
                    midi::cckind::channelvolume => *(self.params[param_name::gain as usize]) = mm.cc_value(),
                    _ => println!("Don't understand cc midi message", )
                }
            }
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
        unsafe{
            *(self.params[param_name::gain as usize])*self.synth.get_amp()
        }
    }
}
