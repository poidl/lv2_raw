extern crate libc;
use std::ptr;
use std::mem;
mod lv2;

pub enum PortIndex {
        MIDI_IN = 0,
        AUDIO_OUT = 1
}

#[repr(C)]
struct Synth {
    gain: *const f32,
    input: *const f32,
    output: *mut f32
}


impl lv2::LV2Descriptor {
    pub extern fn instantiate(_descriptor: *const lv2::LV2Descriptor, _rate: &mut f64, _bundle_path: *const u8, _features: *const lv2::LV2Feature)
                                -> lv2::Lv2handle {
                                let ptr: *mut libc::c_void;
                                unsafe{
                                    ptr = libc::malloc(mem::size_of::<Synth>() as libc::size_t) as *mut libc::c_void;
                                }
                                return ptr;
    }
    pub extern fn connect_port(handle: lv2::Lv2handle, port: u32, data: *mut libc::c_void) {}
    pub extern fn activate(_instance: lv2::Lv2handle) {}
    pub extern fn run(instance: lv2::Lv2handle, n_samples: u32) {}

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

static S: &'static [u8] = b"http://example.org/eg-amp_rust\0";
static mut desc: lv2::LV2Descriptor = lv2::LV2Descriptor {
    amp_uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
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
        desc.amp_uri = ptr;
        return &desc as *const lv2::LV2Descriptor
        }
    }
}
