
use std::collections::HashMap;
use voice;
use voice::*;
use std::ptr;

pub struct Synth {
    fs: f32,
    voice: voice::Voice,
}

impl Synth {
    pub fn new() -> Synth {
        Synth {
            fs: 0f32,
            voice: voice::Voice::new(),
        }
    }
    pub fn set_fs(&mut self, fs: f64) {
        self.voice.set_fs(fs);
    }
    pub fn noteon(&mut self, f0: f32, vel: f32) {
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
    pub fn noteoff(&mut self) {
        self.voice.on = false;
    }
    // fn controlevent(&mut self, paramId, paramVal) {
    //     self.updateParam
    // }
    pub fn get_amp(&mut self) -> f32 {
        self.voice.get_amp()
    }
}
