#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

extern crate libc;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
use plugin;
use lv2;
use midi;
use midi::*;
use plugin::*;
use std::str;

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
}

impl  Lv2SynthPlugin {
    pub fn new() -> Lv2SynthPlugin {
        // let np = ptr::null();
        let mut lv2plugin = Lv2SynthPlugin {
            map: ptr::null(),
            in_port: ptr::null(),
            output: ptr::null_mut(),
            uris: Synthuris::new(),
            plugin: plugin::SynthPlugin::new(),
        };
        // TODO: this is to avoid needing to access lv2plugin.plugin in lv2::LV2Descriptor::connect_port()
        lv2plugin.output = lv2plugin.plugin.audio_out;
        lv2plugin
    }
    pub fn mapfeatures(&mut self, hostfeatures: *const (*const lv2::LV2Feature)) -> Result<&'static str, &'static str> {
        let requiredfeature = "http://lv2plug.in/ns/ext/urid#map";
        let mut x: isize = 0;
        let mut done = false;
        unsafe{
            while !done {

                let fptr: *const lv2::LV2Feature = *hostfeatures.offset(x);
                if fptr.is_null() {
                    // host doesn't provide feature
                    println!("Missing feature \"{}\"", requiredfeature);
                    return Err("missing feature")
                }
                let uriptr = (*fptr).uri;
                let buf = CStr::from_ptr(uriptr).to_bytes();
                let s: &str = str::from_utf8(buf).unwrap();
                println!("uri: {}", s);
                if s == requiredfeature {
                    self.map = (*fptr).data;
                    done=true;
                    println!{" -> obtained urid#map from host"}
                }
                x = x+1;
            }
        }
        Ok("mapping done")
    }
    pub fn seturis(&mut self) {
        unsafe{
            let s = "http://lv2plug.in/ns/ext/midi#MidiEvent";
            let cstr = CString::new(s).unwrap();
            let lv2_midi_midi_event = cstr.as_ptr();
            self.uris.midi_event = ((*self.map).map)((*self.map).handle, lv2_midi_midi_event);
        }
    }
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
