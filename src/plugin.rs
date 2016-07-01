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

pub enum ParamName {
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
    pub fn noteon(&mut self, f0: f32, vel: f32) {
        self.synth.noteon(f0, vel);
    }
    pub fn noteoff(&mut self) {
        self.synth.noteoff();
    }
    pub fn set_fs(&mut self, fs: f64) {
        self.synth.set_fs(fs);
    }
    pub fn get_amp(&mut self) -> f32 {
        unsafe {
            let g = *(self.params[ParamName::Gain as usize]);
            // println!("g: {}", g);
            g * self.synth.get_amp()
            // self.synth.get_amp()
        }
    }
}
