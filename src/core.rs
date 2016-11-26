use libc;

pub trait LV2HandleNew<'a> {
    fn connect_port(&mut self, _port: u32, _data: &'a mut [f32]) {}
    fn activate(&mut self) {}
    fn run(&mut self, _n_samples: u32) {}
    fn deactivate(&mut self) {}
    fn cleanup(&mut self) {}
}

pub type LV2Handle = *mut libc::c_void;

#[repr(C)]
pub struct LV2Feature {
    pub uri: *const libc::c_char,
    pub data: *mut libc::c_void, // pub data: *mut LV2UridMap,
}

#[repr(C)]
pub struct LV2Descriptor {
    pub uri: *const libc::c_char,
    pub instantiate: extern "C" fn(descriptor: *const LV2Descriptor,
                                   rate: f64,
                                   bundle_path: *const libc::c_char,
                                   features: *const (*const LV2Feature))
                                   -> LV2Handle,
    pub connect_port: extern "C" fn(handle: LV2Handle, port: u32, data: *mut libc::c_void),
    pub activate: extern "C" fn(instance: LV2Handle),
    pub run: extern "C" fn(instance: LV2Handle, n_samples: u32),
    pub deactivate: extern "C" fn(instance: LV2Handle),
    pub cleanup: extern "C" fn(instance: LV2Handle),
    pub extension_data: extern "C" fn(uri: *const u8) -> (*const libc::c_void),
}
