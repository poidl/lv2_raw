#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
// #![allow(non_snake_case)]

// #![feature(alloc)]
//#![feature(heap_api)]
// #![feature(unique)]


//mod tests;
pub mod utils;
mod lv2;
mod oscillators;
mod voice;
//mod heapslice;

extern crate libc;
use std::ptr;
use std::mem;
use std::str;
use std::ffi::CString;
use std::ffi::CStr;
use utils::*;
use oscillators::*;
use voice::*;

// struct Synth {
//     output: *mut f32,
//     fs: f64,
//     voice: Voice,
//     osc1: OscBasic
//     // osc2: Osc
// }

// impl Synth {
//     fn new(fs: f64) -> Synth {
//         Synth {
//             output: ptr::null_mut(),
//             fs: fs,
//             voice: Voice { f0: 0f64, vel: 0u8, on: false},
//             osc1: OscBasic { fs: fs, phase: 0, dphase: 0}
//         }
//     }
//     fn noteon (&mut self, f0: f64, vel: u8) {
//         self.voice.f0 = f0;
//         self.voice.vel = vel;
//         self.voice.on = true;
//         self.osc1.reset_phase();
//         self.osc1.set_dphase(self.voice.f0);
//     }
//     fn run(&mut self, n_samples: u32)  {
//
//
//     }
// }



// impl lv2::LV2Descriptor {

//     pub extern fn run(instance: lv2::Lv2handle, n_samples: u32) {
//         unsafe{
//             let synth = instance as *mut Synth;
//             let uris = &mut (*synth).uris;
//             let seq = (*synth).in_port;
//             let output = (*synth).output;
//             // pointer to 1st event body
//             let mut ev: *const lv2::Lv2AtomEvent  = lv2::lv2_atom_sequence_begin(&(*seq).body);
//             // loop through event sequence
//             while !lv2::lv2_atom_sequence_is_end(&(*seq).body, (*seq).atom.size, ev) {
//                 // check if event is midi
//                 if (*ev).body.mytype == (*uris).midi_event {
//
//                     // pointer to midi event data
//                     let msg: *const u8 = ev.offset(1) as *const u8;
//                     // frameindex of eventstart. In jalv this is relative to currently processed buffer chunk of length n_samples
//                     let istart = (*ev).time_in_frames as u32;
//
//                     match lv2::lv2_midi_message_type(msg) {
//
//                         // note on event
//                         lv2::Lv2MidiMessageType::Lv2MidiMsgNoteOn => {
//                             (*synth).noteison = true;
//                             let f0 = f0_from_midimsg(msg);
//                             (*synth).f0 = f0;
//                             (*synth).currentmidivel = *msg.offset(2);
//                             let coef = 1.0 as f32;
//
//                             (*synth).osc.reset();
//                             (*synth).osc.set_dphase(f0,(*synth).fs);
//
//                             // TODO don't set fs here
//                             (*synth).oscST.reset((*synth).fs);
//                             (*synth).oscST.set_f0fn(f0);
//
//                             for i in istart-1..n_samples {
//                                 // let amp = (*synth).osc.get() as f32;
//                                 let amp = (*synth).oscST.get() as f32;
//                                 *output.offset(i as isize) = amp;
//                             }
//                         }
//
//                         // note off event
//                         lv2::Lv2MidiMessageType::Lv2MidiMsgNoteOff => {
//                             (*synth).noteison = false;
//                             (*synth).makesilence = true;
//                             for i in istart-1..n_samples {
//                                 let amp = 0.0 as f32;
//                                 *output.offset(i as isize) = amp as f32;
//                             }
//                         }
//
//                         _ => {
//                             println!("DON'T UNDERSTAND MESSAGE")
//                         }
//
//                     }
//                 }
//                 ev = lv2::lv2_atom_sequence_next(ev);
//             }
//
//             if (*synth).noteison {
//                 let coef = 1.0 as f32;
//                 let f0 = (*synth).f0;
//
//                 for i in 0..n_samples {
//                     // let amp = (*synth).osc.get();
//                     let amp = (*synth).oscST.get();
//                     *output.offset(i as isize) = (amp as f32) * coef;
//                 }
//
//             } else if (*synth).makesilence {
//                 (*synth).makesilence = false;
//                 for i in 0..n_samples {
//                     let amp = 0.0;
//                     *output.offset(i as isize) = amp as f32;
//                 }
//             }
//
//         }
//
//     }
//
//     pub extern fn deactivate(_instance: lv2::Lv2handle) {}
//     pub extern fn cleanup(instance: lv2::Lv2handle) {
//
//         unsafe{
//             //ptr::read(instance as *mut Amp); // no need for this?
//             libc::free(instance  as lv2::Lv2handle)
//         }
//     }
//     pub extern fn extension_data(_uri: *const u8)-> (*const libc::c_void) {
//                             ptr::null()
//     }
// }
//
// static S: &'static [u8] = b"http://example.org/yassy\0";
// static mut desc: lv2::LV2Descriptor = lv2::LV2Descriptor {
//     uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
//     instantiate: lv2::LV2Descriptor::instantiate,
//     connect_port: lv2::LV2Descriptor::connect_port,
//     activate: lv2::LV2Descriptor::activate,
//     run: lv2::LV2Descriptor::run,
//     deactivate: lv2::LV2Descriptor::deactivate,
//     cleanup: lv2::LV2Descriptor::cleanup,
//     extension_data: lv2::LV2Descriptor::extension_data
// };
//
// #[no_mangle]
// pub extern fn lv2_descriptor(index:i32) -> *const lv2::LV2Descriptor {
//     if index != 0 {
//         return ptr::null();
//     } else {
//         // credits to ker on stackoverflow: http://stackoverflow.com/questions/31334356/static-struct-with-c-strings-for-lv2-plugin (duplicate) or http://stackoverflow.com/questions/25880043/creating-a-static-c-struct-containing-strings
//         let ptr = S.as_ptr() as *const libc::c_char;
//         unsafe {
//         desc.uri = ptr;
//         return &desc as *const lv2::LV2Descriptor
//         }
//     }
// }
//
// struct Synthuris {
//     midi_event: lv2::Lv2urid
// }
//
// fn map_synth_uris(map: *const lv2::Lv2uridMap, uris: &mut Synthuris) {
//     let s = "http://lv2plug.in/ns/ext/midi#MidiEvent";
//     let cstr = CString::new(s).unwrap();
//     let lv2_midi_midi_event = cstr.as_ptr();
//     //mem::forget(cstr);
//     unsafe{
//         uris.midi_event = ((*map).map)((*map).handle, lv2_midi_midi_event);
//     }
// }
//
// fn f0_from_midimsg(msg: *const u8) -> f64 {
//     // A3 has midi number 56
//     // Frequencies are calculated with the formula
//     // f0 = {[(2)^1/12]^n} * 220 Hz,
//     // where n is the number of half steps from A3
//     unsafe{
//         let i = *msg.offset(1);
//         let f0 = (2.0f64.powf((((i as i8)-57) as f64)/12.0))*220.0;
//         // println!("FREQ: {}", freq);
//         return f0
//     }
// }
//
// fn amp(isample: u32, f0: f64, fs: f64) -> f64 {
//     let lam = fs/f0;
//     // println!("fs: {}", fs);
//     return (2.0*std::f64::consts::PI*((isample as f64)/lam)).sin()
// }
//
