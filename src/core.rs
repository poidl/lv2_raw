use libc;
use urid::*;

pub type LV2Handle = *mut libc::c_void;

#[repr(C)]
pub struct LV2Feature {
    pub uri: *const libc::c_char,
    pub data: *mut LV2UridMap
}

#[repr(C)]
pub struct LV2Descriptor {
    pub uri: *const  libc::c_char,
    pub instantiate: extern fn(descriptor: *const LV2Descriptor,rate: f64, bundle_path: *const libc::c_char, 	features: *const (*const LV2Feature) )
                                -> LV2Handle,
    pub connect_port: extern fn(handle: LV2Handle, port: u32, data: *mut libc::c_void),
    pub activate: extern fn(instance: LV2Handle),
    pub run: extern fn(instance: LV2Handle, n_samples: u32),
    pub deactivate: extern fn(instance: LV2Handle),
    pub cleanup: extern fn(instance: LV2Handle),
    pub extension_data: extern fn(uri: *const u8)-> (*const libc::c_void),
}
