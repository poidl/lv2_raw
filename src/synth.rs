use voice;

pub trait isSynth {
    fn midievent(&self, *const u8);
    fn getAmp(&self) -> f32;
}

pub struct Synth {
    fs: f64,
    voice: voice::Voice,
    // osc2: Osc
}

impl isSynth for Synth {
    fn midievent(&self, msg: *const u8) {
        println!("f0: {}", f0_from_midimsg(msg))
    }
    fn getAmp(&self) -> f32 {
        0.5f32
    }
}

fn f0_from_midimsg(msg: *const u8) -> f64 {
    // A3 has midi number 56
    // Frequencies are calculated with the formula
    // f0 = {[(2)^1/12]^n} * 220 Hz,
    // where n is the number of half steps from A3
    unsafe{
        let i = *msg.offset(1);
        let f0 = (2.0f64.powf((((i as i8)-57) as f64)/12.0))*220.0;
        // println!("FREQ: {}", freq);
        return f0
    }
}

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
