extern crate libc;
use std::ptr;
use std::mem;

pub type Lv2handle = *mut libc::c_void;

pub enum PortIndex {
        AmpGain = 0,
        AmpInput= 1,
        AmpOutput = 2
}

#[repr(C)]
pub struct LV2Feature {
    uri: *const u8,
    data: *mut libc::c_void
}

#[repr(C)]
pub struct LV2Descriptor {
    amp_uri: *const  libc::c_char,
    instantiate: extern fn(descriptor: *const LV2Descriptor, rate:  &mut f64,
                            bundle_path: *const u8, features: *const LV2Feature)
                                -> Lv2handle,
    connect_port: extern fn(handle: Lv2handle, port: PortIndex, data: *mut libc::c_void),
    activate: extern fn(instance: Lv2handle),
    run: extern fn(instance: Lv2handle, n_samples: u32),
    deactivate: extern fn(instance: Lv2handle),
    cleanup: extern fn(instance: Lv2handle),
    extension_data: extern fn(uri: *const u8)-> (*const libc::c_void),
}

#[repr(C)]
struct Amp {
    gain: *const f32,
    input: *const f32,
    output: *mut f32
}

impl LV2Descriptor {
    pub extern fn instantiate(_descriptor: *const LV2Descriptor, _rate: &mut f64, _bundle_path: *const u8, _features: *const LV2Feature)
                                -> Lv2handle {
                                let ptr: *mut libc::c_void;
                                unsafe{
                                    ptr = libc::malloc(mem::size_of::<Amp>() as libc::size_t) as *mut libc::c_void;
                                }
                                return ptr;
    }
    pub extern fn connect_port(handle: Lv2handle, port: PortIndex, data: *mut libc::c_void) {
        let amp: *mut Amp = handle as *mut Amp;
        match port {
            PortIndex::AmpGain => unsafe{ (*amp).gain = data  as *const f32 }, // data may be NULL pointer, so don't dereference!
            PortIndex::AmpInput => unsafe{ (*amp).input = data as *const f32 },
            PortIndex::AmpOutput => unsafe{ (*amp).output = data as *mut f32 },
        }
    }
    pub extern fn activate(_instance: Lv2handle) {}
    pub extern fn run(instance: Lv2handle, n_samples: u32) {
        let amp = instance as *const Amp;
        let gain = unsafe{ *((*amp).gain) };
        let input: *const f32 = unsafe{  (*amp).input };
        let output: *mut f32 = unsafe{ (*amp).output };

        let mut coef:  f32;
        match gain > -90.0 {
            true    =>  coef =(10.0 as f32).powf(gain*0.05),
            false =>  coef = 0.0
        }

        unsafe{
            for x in 0..n_samples-1 {
                *output.offset(x as isize) = *input.offset(x as isize) * coef;
            }
        }
    }

    pub extern fn deactivate(_instance: Lv2handle) {}
    pub extern fn cleanup(instance: Lv2handle) {

        unsafe{
            //ptr::read(instance as *mut Amp); // no need for this?
            libc::free(instance  as Lv2handle)
        }
    }
    pub extern fn extension_data(_uri: *const u8)-> (*const libc::c_void) {
                            ptr::null()
    }
}

static S: &'static [u8] = b"http://example.org/eg-amp_rust\0";
static mut desc: LV2Descriptor = LV2Descriptor {
    amp_uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
    instantiate: LV2Descriptor::instantiate,
    connect_port: LV2Descriptor::connect_port,
    activate: LV2Descriptor::activate,
    run: LV2Descriptor::run,
    deactivate: LV2Descriptor::deactivate,
    cleanup: LV2Descriptor::cleanup,
    extension_data: LV2Descriptor::extension_data
};

#[no_mangle]
pub extern fn lv2_descriptor(index:i32) -> *const LV2Descriptor {
    if index != 0 {
        return ptr::null();
    } else {
        // credits to ker on stackoverflow: http://stackoverflow.com/questions/31334356/static-struct-with-c-strings-for-lv2-plugin (duplicate) or http://stackoverflow.com/questions/25880043/creating-a-static-c-struct-containing-strings
        let ptr = S.as_ptr() as *const libc::c_char;
        unsafe {
        desc.amp_uri = ptr;
        return &desc as *const LV2Descriptor
        }
    }
}
