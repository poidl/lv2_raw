#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
// #![allow(non_snake_case)]

// #![feature(alloc)]
//#![feature(heap_api)]
// #![feature(unique)]

//mod tests;
// pub mod utils;
mod lv2;
mod oscillator;
mod voice;
mod lv2_plugin;
mod synth;
//mod heapslice;

extern crate libc;
extern crate midi;
use std::ptr;
use std::mem;
use std::str;
use std::ffi::CString;
use std::ffi::CStr;
// use utils::*;
use oscillator::*;
use voice::*;
use lv2_plugin::*;



impl lv2::LV2Descriptor {
pub extern fn instantiate( _descriptor: *const lv2::LV2Descriptor , fs: f64, bundle_path: *const libc::c_char , features: *const (*const lv2::LV2Feature),) -> lv2::Lv2handle {
        unsafe{
            let ptr = libc::calloc(1,mem::size_of::<lv2_plugin::Lv2SynthPlugin>() as libc::size_t);
            if ptr.is_null() {
                panic!("failed to allocate memory");
            }

            let mut map = (*(ptr  as *mut lv2_plugin::Lv2SynthPlugin)).map;

            let uridmapstr = "http://lv2plug.in/ns/ext/urid#map";
            let mut x: isize = 0;
            let mut done = false;
            while !done {

                let fptr: *const lv2::LV2Feature = *features.offset(x);
                if fptr.is_null() {
                    // host doesn't provide feature
                    libc::free(ptr as *mut libc::c_void);
                    println!("Missing feature \"{}\"", uridmapstr);
                    return std::ptr::null_mut();
                }
                let uriptr = (*fptr).uri;
                let buf = CStr::from_ptr(uriptr).to_bytes();
                let s: &str = str::from_utf8(buf).unwrap();
                println!("uri: {}", s);
                if s == uridmapstr {
                    map = (*fptr).data;
                    done=true;
                    println!{" -> obtained urid#map from host"}
                }
                x = x+1;
            }
            let s = "http://lv2plug.in/ns/ext/midi#MidiEvent";
            let cstr = CString::new(s).unwrap();
            let lv2_midi_midi_event = cstr.as_ptr();
            (*(ptr  as *mut lv2_plugin::Lv2SynthPlugin)).uris.midi_event = ((*map).map)((*map).handle, lv2_midi_midi_event);
            (*(ptr  as *mut lv2_plugin::Lv2SynthPlugin)).set_fs(fs);
            ptr
        }
    }

    pub extern fn connect_port(handle: lv2::Lv2handle, port: u32, data: *mut libc::c_void) {
        let synth: *mut lv2_plugin::Lv2SynthPlugin = handle as *mut lv2_plugin::Lv2SynthPlugin;
        unsafe {
            (*synth).connect_port(port,data)
        }
    }
    pub extern fn activate(_instance: lv2::Lv2handle) {}

    pub extern fn run(instance: lv2::Lv2handle, n_samples: u32) {
        unsafe{
            let synth = instance as *mut lv2_plugin::Lv2SynthPlugin;
            let uris = &mut (*synth).uris;
            let seq = (*synth).in_port;
            let output = (*synth).output;
            // pointer to 1st event body
            let mut ev: *const lv2::Lv2AtomEvent  = lv2::lv2_atom_sequence_begin(&(*seq).body);
            // loop through event sequence
            while !lv2::lv2_atom_sequence_is_end(&(*seq).body, (*seq).atom.size, ev) {
                // check if event is midi
                if (*ev).body.mytype == (*uris).midi_event {
                    // pointer to midi event data
                    let msg: &u8 = &*(ev.offset(1) as *const u8);
                    (*synth).midievent(msg);

                    let istart = (*ev).time_in_frames as u32;

                    for i in istart..n_samples {
                        let amp = (*synth).get_amp();
                        // println!("Amp: {}", amp);
                        *output.offset(i as isize) = amp;
                    }
                }
                ev = lv2::lv2_atom_sequence_next(ev);
            }
            for i in 0..n_samples {
                let amp = (*synth).get_amp();
                // println!("Amp: {}", amp);
                *output.offset(i as isize) = amp;
            }
        }
    }

    pub extern fn deactivate(_instance: lv2::Lv2handle) {}

    pub extern fn cleanup(instance: lv2::Lv2handle) {

        unsafe{
            //ptr::read(instance as *mut Amp); // no need for this?
            libc::free(instance  as lv2::Lv2handle)
        }
    }
    pub extern fn extension_data(_uri: *const u8)-> (*const libc::c_void) {
                            ptr::null()
    }
}

static S: &'static [u8] = b"http://example.org/yassy\0";

static mut desc: lv2::LV2Descriptor = lv2::LV2Descriptor {
    uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
    instantiate: lv2::LV2Descriptor::instantiate,
    connect_port: lv2::LV2Descriptor::connect_port,
    activate: lv2::LV2Descriptor::activate,
    run: lv2::LV2Descriptor::run,
    deactivate: lv2::LV2Descriptor::deactivate,
    cleanup: lv2::LV2Descriptor::cleanup,
    extension_data: lv2::LV2Descriptor::extension_data
};

#[no_mangle]
pub extern fn lv2_descriptor(index:i32) -> *const lv2::LV2Descriptor {
    if index != 0 {
        return ptr::null();
    } else {
        // credits to ker on stackoverflow: http://stackoverflow.com/questions/31334356/static-struct-with-c-strings-for-lv2-plugin (duplicate) or http://stackoverflow.com/questions/25880043/creating-a-static-c-struct-containing-strings
        let ptr = S.as_ptr() as *const libc::c_char;
        unsafe {
        desc.uri = ptr;
        return &desc as *const lv2::LV2Descriptor
        }
    }
}
