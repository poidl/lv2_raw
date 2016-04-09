use voice;
use midi;
use midi::*;
use voice::*;

pub trait isSynth {
    fn set_fs(&mut self, f64);
    fn midievent(&mut self, &u8);
    fn get_amp(&mut self) -> f32;
}

pub struct Synth {
    fs: f32,
    voice: voice::Voice
}

impl isSynth for Synth {
    fn midievent(&mut self, msg: &u8) {
        let mm = msg as midi::MidiMessage;
        if mm.noteon() {
            println!("mm.noteon(): {}", mm.noteon());
            self.voice.on = true;
            self.voice.f0 = mm.f0();
            // let a=-2.302587f32;
            // let b=0.0953105f32;
            // let c= 1f32/(b-a);
            // plot((1/(log(1.01)-log(0.01)))*(log(y+0.01)-log(0.01)))
            // self.voice.vel = c*( (mm.vel()+0.01).ln()-a );
            self.voice.vel = mm.vel();
            self.voice.initialize();
        } else if mm.noteoff() {
            println!("mm.noteff(): {}", mm.noteoff());
            self.voice.on = false;
        } else {
            println!("Don't understand midi message", );
        }
    }
    fn set_fs(&mut self, fs: f64) {
        self.voice.set_fs(fs);
    }
    fn get_amp(&mut self) -> f32 {
        self.voice.get_amp()
    }
}
