use libc;
use std::ptr;
use std::mem;
use std::slice;
// use std::marker::PhantomData;
pub trait LV2HandleNew<'a> {
    fn initialize(&mut self) {}
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


pub extern "C" fn instantiate<'a, T: LV2HandleNew<'a>>(_descriptor: *const LV2Descriptor,
                                                       _rate: f64,
                                                       _bundle_path: *const i8,
                                                       _features: *const *const LV2Feature)
                                                       -> LV2Handle {

    let ptr: *mut libc::c_void;
    unsafe {
        ptr = libc::malloc(mem::size_of::<T>() as libc::size_t) as *mut libc::c_void;
        let plgptr = ptr as *mut T;
        (*plgptr).initialize()
    }
    return ptr;
}

pub extern "C" fn connect_port<'a, T: LV2HandleNew<'a>>(handle: LV2Handle,
                                                        port: u32,
                                                        data: *mut libc::c_void) {
    let d = data as *mut f32;
    let plgptr = handle as *mut T;
    unsafe {
        // TODO: This should be sample_count. How do we get that number? During initialization?
        let bs: &mut [f32] = slice::from_raw_parts_mut(d, 256 * mem::size_of::<f32>());
        (*plgptr).connect_port(port, bs)
    }
}

pub extern "C" fn activate(_instance: LV2Handle) {}
pub extern "C" fn run<'a, T: LV2HandleNew<'a>>(instance: LV2Handle, n_samples: u32) {
    let plgptr = instance as *mut T;
    unsafe { (*plgptr).run(n_samples) }
}

pub extern "C" fn deactivate(_instance: LV2Handle) {}
pub extern "C" fn cleanup(instance: LV2Handle) {

    unsafe {
        // ptr::read(instance as *mut Amp); // no need for this?
        libc::free(instance as LV2Handle)
    }
}
pub extern "C" fn extension_data(_uri: *const u8) -> (*const libc::c_void) {
    ptr::null()
}