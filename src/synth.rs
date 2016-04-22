
use std::collections::HashMap;
use voice;
use voice::*;

pub trait isSynth {
    fn set_fs(&mut self, f64);
    fn noteon(&mut self, f32, f32);
    fn noteoff(&mut self);
    // fn controlevent(&mut self, &u8);
    fn get_amp(&mut self) -> f32;
}

// pub enum Param {
//     gain {idx: u32, val: f32, ptr: *mut f32},
//     // osctype {val: i32, ptr: *mut f32}
// }
//
// impl Param {
//     pub fn connect(&mut self, data: *mut f32) {
//         match *self {
//             Param::gain {idx, val, mut ptr} => ptr = data,
//             // Param::osctype {val, mut ptr} => ptr = data,
//         }
//     }
//     pub fn get(&self) -> f32 {
//         match *self {
//             Param::gain {idx, val, ptr} => unsafe{ *ptr },
//             // Param::osctype {val, mut ptr} => ptr = data,
//         }
//     }
// }

// struct SynthParams {
//     gain: Param::gain,
//     // osctype: Param::osctype
// }
// let mut book_reviews = HashMap::new();
//
// // review some books.
// book_reviews.insert("Adventures of Huckleberry Finn",    "My favorite book.");
// book_review
// pub enum SynthParams {
//     gain,
// }

pub struct Synth<'a> {
    fs: f32,
    voice: voice::Voice,
    gain: f32,
    pub params: [&'a f32;1]
}

enum param_name {
    gain,
}

// pub struct SynthLv2Params {
//     gain: *const f32
// }

impl<'a> isSynth for Synth<'a> {
    fn set_fs(&mut self, fs: f64) {
        self.voice.set_fs(fs);
    }
    fn noteon(&mut self, f0: f32, vel: f32) {
        self.voice.on = true;
        self.voice.f0 = f0;
        // let a=-2.302587f32;
        // let b=0.0953105f32;
        // let c= 1f32/(b-a);
        // plot((1/(log(1.01)-log(0.01)))*(log(y+0.01)-log(0.01)))
        // self.voice.vel = c*( (mm.vel()+0.01).ln()-a );
        self.voice.vel = vel;
        self.voice.initialize();
    }
    fn noteoff(&mut self) {
        self.voice.on = false;
    }
    // fn controlevent(&mut self, paramId, paramVal) {
    //     self.updateParam
    // }
    fn get_amp(&mut self) -> f32 {
        println!("gain: {}", *(self.params[param_name::gain as usize]));
        *(self.params[param_name::gain as usize])*self.voice.get_amp()
    }
    // fn set_param(&mut self, id, val) {
    //     self.params.gain = g;
    // }
}
