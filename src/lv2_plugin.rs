#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use std::ffi::CString;
use synth;
use synth::isSynth;
use lv2;

pub trait isLv2SynthPlugin {
    fn midievent(&self, *const u8);
    fn getAmp(&self) -> f32;
}

pub struct Synthuris {
    pub midi_event: lv2::Lv2urid
}

#[repr(C)]
pub struct Lv2SynthPlugin {
    pub map: *const lv2::Lv2uridMap,
    pub in_port: *const lv2::LV2_Atom_Sequence,
    pub output: *mut f32,
    pub uris: Synthuris,
    // pub synth: synth::Synth
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

// impl isLv2SynthPlugin for Lv2SynthPlugin {
//     fn midievent(&self, msg: *const u8) {
//         self.synth.midievent(msg);
//     }
//     fn getAmp(&self) -> f32 {
//         self.synth.getAmp()
//     }
// }
