// This is a translation of http://lv2plug.in/git/cgit.cgi/lv2.git/tree/plugins/eg-amp.lv2/amp.c
// from C into Rust by S. Riha (2015)
// Read the README.txt of the original code:
// http://lv2plug.in/git/cgit.cgi/lv2.git/tree/plugins/eg-amp.lv2/README.txt
// The copyright notice of the original C file:

// Copyright 2006-2011 David Robillard <d@drobilla.net>
// Copyright 2006 Steve Harris <steve@plugin.org.uk>
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//

// Notes:
// 2015/07/11: I just started programming in Rust and I'm rather new to low level programming in
// general (memory management, etc.). I'm sure there is a lot of room for improvement here, if
// you have any ideas please let me know (hoitaus@gmail.com)
// 2015/07/11: It is currently not possible to allocate a struct that contains a C string. See
// comment in fn lv2_descriptor().

extern crate libc;
extern crate lv2;
use std::ptr;
use std::mem;
use lv2::*;

#[repr(C)]
struct Amp {
    gain: *const f32,
    input: *const f32,
    output: *mut f32,
}

// have to define new type. Otherwise error: "cannot define inherent impl for a type outside of the crate where the type is defined; define and implement a trait or new type instead"
struct Descriptor(lv2::LV2Descriptor);

impl Descriptor {
    pub extern "C" fn instantiate(_descriptor: *const lv2::LV2Descriptor,
                                  _rate: f64,
                                  _bundle_path: *const i8,
                                  _features: *const *const lv2::LV2Feature)
                                  -> lv2::LV2Handle {
        let ptr: *mut libc::c_void;
        unsafe {
            ptr = libc::malloc(mem::size_of::<Amp>() as libc::size_t) as *mut libc::c_void;
        }
        return ptr;
    }
    pub extern "C" fn connect_port(handle: lv2::LV2Handle, port: u32, data: *mut libc::c_void) {
        let amp: *mut Amp = handle as *mut Amp;
        match port {
            0 => unsafe { (*amp).gain = data as *const f32 }, // data may be NULL pointer, so don't dereference!
            1 => unsafe { (*amp).input = data as *const f32 },
            2 => unsafe { (*amp).output = data as *mut f32 },
            _ => panic!("Not a valid PortIndex: {}", port)
        }
    }
    pub extern "C" fn activate(_instance: lv2::LV2Handle) {}
    pub extern "C" fn run(instance: lv2::LV2Handle, n_samples: u32) {
        let amp = instance as *const Amp;
        let gain = unsafe { *((*amp).gain) };
        let input: *const f32 = unsafe { (*amp).input };
        let output: *mut f32 = unsafe { (*amp).output };

        let coef: f32;
        match gain > -90.0 {
            true => coef = (10.0 as f32).powf(gain * 0.05),
            false => coef = 0.0,
        }

        unsafe {
            for x in 0..n_samples {
                *output.offset(x as isize) = *input.offset(x as isize) * coef;
            }
        }
    }

    pub extern "C" fn deactivate(_instance: lv2::LV2Handle) {}
    pub extern "C" fn cleanup(instance: lv2::LV2Handle) {

        unsafe {
            // ptr::read(instance as *mut Amp); // no need for this?
            libc::free(instance as lv2::LV2Handle)
        }
    }
    pub extern "C" fn extension_data(_uri: *const u8) -> (*const libc::c_void) {
        ptr::null()
    }
}

static S: &'static [u8] = b"http://example.org/eg-amp_rust\0";
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
pub extern "C" fn lv2_descriptor(index: i32) -> *const LV2Descriptor {
    if index != 0 {
        return ptr::null();
    } else {
        // credits to ker on stackoverflow: http://stackoverflow.com/questions/31334356/static-struct-with-c-strings-for-lv2-plugin (duplicate) or http://stackoverflow.com/questions/25880043/creating-a-static-c-struct-containing-strings
        let ptr = S.as_ptr() as *const libc::c_char;
        unsafe {
            desc.uri = ptr;
            return &desc as *const LV2Descriptor;
        }
    }
}