extern crate libc;
extern crate midi;
extern crate lv2;

mod oscillator;
mod voice;
mod lv2_plugin;
mod synth;
pub mod plugin;

use std::ptr;
use std::mem;

// have to define new type. Otherwise error: "cannot define inherent impl for a type outside of the crate where the type is defined; define and implement a trait or new type instead"
struct Descriptor(lv2::LV2Descriptor);

impl Descriptor {
    pub extern fn instantiate( _descriptor: *const lv2::LV2Descriptor , fs: f64, _bundle_path: *const libc::c_char, hostfeatures: *const (*const lv2::LV2Feature),) -> lv2::LV2Handle {

        let mut bx = Box::new(lv2_plugin::Lv2SynthPlugin::new());
        let featureptr = lv2::mapfeature(hostfeatures,"http://lv2plug.in/ns/ext/urid#map");
        match featureptr {
            Ok(fp) => bx.map =fp as *mut lv2::LV2UridMap,
            _ => return ptr::null_mut()
        }
        bx.seturis();
        bx.set_fs(fs);
        let ptr = (&*bx as *const lv2_plugin::Lv2SynthPlugin) as *mut libc::c_void;
        mem::forget(bx);
        ptr
    }

    pub extern fn connect_port(handle: lv2::LV2Handle, port: u32, data: *mut libc::c_void) {
        let synth: *mut lv2_plugin::Lv2SynthPlugin = handle as *mut lv2_plugin::Lv2SynthPlugin;
        unsafe {
            (*synth).connect_port(port,data)
        }
    }
    pub extern fn activate(_instance: lv2::LV2Handle) {}

    pub extern fn run(instance: lv2::LV2Handle, n_samples: u32) {
        unsafe{
            let synth = instance as *mut lv2_plugin::Lv2SynthPlugin;
            let uris = &mut (*synth).uris;
            let seq = (*synth).in_port;
            let output = (*synth).output;
            // pointer to 1st event body
            let mut ev: *const lv2::Lv2AtomEvent  = lv2::lv2_atom_sequence_begin(&(*seq).body);

            let mut mq = false; // midi is queued
            let mut ievent = 0;

            for i in 0..n_samples {
                // at i=0, search for the first midi event and get index
                if !mq {
                    while !lv2::lv2_atom_sequence_is_end(&(*seq).body, (*seq).atom.size, ev) {
                        mq = (*ev).body.mytype == (*uris).midi_event;
                        if !mq {
                            ev = lv2::lv2_atom_sequence_next(ev);
                        } else {
                            ievent = (*ev).time_in_frames as u32;
                            break;
                        }
                    }
                }

                // compare midi event index with i
                if mq & (ievent==i) {
                    let msg: &u8 = &*(ev.offset(1) as *const u8);
                    (*synth).midievent(msg);
                    // move to next event and set mq (midi is queued) to false
                    ev = lv2::lv2_atom_sequence_next(ev);
                    mq = false;
                }

                let amp = (*synth).get_amp();
                // println!("Amp: {}", amp);
                *output.offset(i as isize) = amp;

            }
        }
    }

    pub extern fn deactivate(_instance: lv2::LV2Handle) {}

    pub extern fn cleanup(instance: lv2::LV2Handle) {

        unsafe{
            //ptr::read(instance as *mut Amp); // no need for this?
            libc::free(instance  as lv2::LV2Handle)
        }
    }
    pub extern fn extension_data(_uri: *const u8)-> (*const libc::c_void) {
                            ptr::null()
    }
}

static S: &'static [u8] = b"http://example.org/yassy\0";

static mut desc: lv2::LV2Descriptor = lv2::LV2Descriptor {
    uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
    instantiate: Descriptor::instantiate,
    connect_port: Descriptor::connect_port,
    activate: Descriptor::activate,
    run: Descriptor::run,
    deactivate: Descriptor::deactivate,
    cleanup: Descriptor::cleanup,
    extension_data: Descriptor::extension_data
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


