#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use std::ffi::CString;
use synth;
use synth::isSynth;
use lv2;

pub trait isLv2SynthPlugin: isSynth {}

pub struct Synthuris {
    pub midi_event: lv2::Lv2urid
}

#[repr(C)]
pub struct Lv2SynthPlugin {
    pub map: *const lv2::Lv2uridMap,
    pub in_port: *const lv2::LV2_Atom_Sequence,
    pub output: *mut f32,
    pub uris: Synthuris,
    pub synth: synth::Synth,
    fs: f64
}


impl isSynth for Lv2SynthPlugin {
    fn midievent(&mut self, msg: &u8) {
        self.synth.midievent(msg);
    }
    fn set_fs(&mut self, fs: f64) {
        self.synth.set_fs(fs);
    }
    fn get_amp(&mut self) -> f32 {
        self.synth.get_amp()
    }
}
