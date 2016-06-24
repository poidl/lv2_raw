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

enum ParamName {
    Gain,
}

pub trait HasFs {
    // sample rate
    fn set_fs(&mut self, f64);
}

pub struct SynthPlugin {
    pub midi_in: *const u8,
    pub audio_out: *mut f32,
    pub synth: synth::Synth,
    fs: f64,
    pub params: [*mut f32; 1],
}

impl SynthPlugin {
    pub fn new() -> SynthPlugin {
        SynthPlugin {
            midi_in: &0u8,
            audio_out: &mut 0f32,
            synth: synth::Synth::new(),
            fs: 0f64,
            params: [&mut 0.5f32; 1],
        }
    }

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
                    midi::cckind::channelvolume => {
                        *(self.params[ParamName::Gain as usize]) = mm.cc_value()
                    }
                    _ => println!("Don't understand cc midi message", ),
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
        unsafe {
            // let g = *(self.params[ParamName::Gain as usize]);
            // g * self.synth.get_amp()
            self.synth.get_amp()
        }
    }
}
