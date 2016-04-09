use oscillator::*;

pub trait IsVoice {
    fn set_fs(&mut self, f64);
    fn get_amp(&mut self) -> f32;
    fn initialize(&mut self);
    // fn reset(&mut self);
    // fn getAmp(&self) -> f32;
}

pub struct Voice {
    pub f0: f32,
    pub vel: f32,
    pub on: bool,
    pub osc1: OscBasic
}

impl IsVoice for Voice {
    fn set_fs(&mut self, fs: f64) {
        self.osc1.set_fs(fs);
    }
    fn get_amp(&mut self) -> f32 {
        if self.on {
            self.vel*self.osc1.get_amp()
        } else {
            0.0
        }

    }
    fn initialize(&mut self) {
        self.osc1.set_f0(self.f0);
        self.osc1.reset_phase();
    }
    // self.osc1.reset(&mut self) {
    //     self.osc1.set_f0 = self.f0;
    //     self.osc1.reset_phase();
    // }
}
