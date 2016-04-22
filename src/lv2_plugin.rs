#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

extern crate libc;

use std::ffi::CString;
use std::ptr;
use plugin;
use lv2;
use midi;
use midi::*;
use plugin::*;

// enum PortIndex {
//     MidiIn = 0,
//     AudioOut = 1,
//     Gain = 2
// }
//
// // Unnecessary? See comment in connect_port().
// impl PortIndex {
//     fn from_int(x: u32) -> PortIndex {
//         match x {
//             0 => PortIndex::MidiIn,
//             1 => PortIndex::AudioOut,
//             2 => PortIndex::Gain,
//             _ => panic!("Not a valid PortIndex: {}", x)
//         }
//     }
// }

pub trait isLv2SynthPlugin: {
    fn connect_port(&mut self, u32, *mut libc::c_void);
    fn midievent(&mut self, msg: &u8) ;
    fn set_fs(&mut self, f64);
    fn get_amp(&mut self) -> f32;
    fn map_params(&mut self, u32, *mut libc::c_void);
}

pub struct Synthuris {
    pub midi_event: lv2::Lv2urid
}

impl Synthuris {
    fn new() -> Synthuris {
        Synthuris {
            midi_event: 0 as lv2::Lv2urid
        }
    }
}

#[repr(C)]
pub struct Lv2SynthPlugin {
    pub map: *const lv2::Lv2uridMap,
    pub in_port: *const lv2::LV2_Atom_Sequence,
    pub output: *mut f32,
    pub uris: Synthuris,
    pub plugin: plugin::SynthPlugin,
    fs: f64
}

pub struct Lv2SynthPluginBuilder {
    pub map: *const lv2::Lv2uridMap,
    pub in_port: *const lv2::LV2_Atom_Sequence,
    pub output: *mut f32,
    pub uris: Synthuris,
    pub plugin: plugin::SynthPlugin,
    fs: f64
}

impl  Lv2SynthPluginBuilder {
    fn new() -> Lv2SynthPluginBuilder {
        // let np = ptr::null();
        Lv2SynthPluginBuilder {
            map: ptr::null(),
            in_port: ptr::null(),
            output: ptr::null_mut(),
            uris: Synthuris::new(),
            plugin: plugin::SynthPlugin::new(),
            fs: 0f64
        }
    }
    // fn finalize(&self) -> Lv2SynthPlugin {
    //     Circle { x: self.x, y: self.y, radius: self.radius }
    // }
}


impl isLv2SynthPlugin for Lv2SynthPlugin {
    fn connect_port(&mut self, port: u32, data: *mut libc::c_void) {
        match port {
            0 => unsafe{self.in_port = data  as *const lv2::LV2_Atom_Sequence},
            1 => unsafe{self.output = data as *mut f32 },
            _ => self.map_params(port,data)
        }
    }
    fn map_params(&mut self, port: u32, data: *mut libc::c_void) {
        let nparams = 1;
        let iport = port - 2; //TODO: don't hardcode number of input/output ports
        if (iport <= nparams-1) {
            println!("connecting port: {}", port);
            unsafe{self.plugin.synth.params[iport as usize]= &*(data  as *mut f32) };
            // println!("param: {}",  *(self.synth.params[0]));
        } else {
            panic!("Not a valid PortIndex: {}", iport)
        }
    }
    fn midievent(&mut self, msg: &u8) {
        let mm = msg as midi::MidiMessage;
        self.plugin.midievent(mm)
    }
    fn set_fs(&mut self, fs: f64) {
        self.plugin.set_fs(fs);
    }
    fn get_amp(&mut self) -> f32 {
        self.plugin.get_amp()
    }
}
