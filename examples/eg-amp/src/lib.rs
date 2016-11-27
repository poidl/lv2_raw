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
// use std::marker::PhantomData;

pub struct AmpNew<'a> {
    gain: &'a f32,
    input: &'a [f32],
    output: &'a mut [f32],
}

impl<'a> lv2::LV2HandleNew<'a> for AmpNew<'a> {
    // TODO: this should be a constructor named "instantiate()" or "new()", depending on whether one wants to adopt LV2 or Rust terminology, respectively (i.e. it should not take &mut self as argument, but allocate an AmpNew); But I don't know how to do this properly. The AmpNew struct only contains references, and there are two ways I can think of:
    // 1) Use raw pointers, but I'd like to avoid that since the code looks more elegant with &-references, e.g. I don't have to use "unsafe" when dereferencing.
    // 2) Create "dummy resources" in AmpNew and carry them around with the struct. Then the &-references can initially point to those resources in the constructor, and the compiler doesn't complain because the resources don't go out of scope when the constructor returns. But that would be awkward, since these resources would never get used, because the actual resources are provided by the host.
    // For now, initialize() is a placeholder function that doesn't do anything. More complicated plugins may scan host features, set a sample rate, etc.
    fn initialize(&mut self) {}
    fn connect_port(&mut self, port: u32, data: &'a mut [f32]) {
        match port {
            0 => self.gain = &data[0] as &f32, // data may be NULL pointer, so don't dereference!
            1 => self.input = data as &'a [f32],
            2 => self.output = data,
            _ => panic!("Not a valid PortIndex: {}", port),
        }
    }
    fn activate(&mut self) {}
    fn run(&mut self, n_samples: u32) {

        let coef: f32;
        match *self.gain > -90.0 {
            true => coef = (10.0 as f32).powf(self.gain * 0.05),
            false => coef = 0.0,
        }
        for x in 0..n_samples {
            let i = x as usize;
            self.output[i] = self.input[i] * coef;
        }

    }
    fn deactivate(&mut self) {}
    fn cleanup(&mut self) {}
}

type Newtype<'a> = AmpNew<'a>;

static S: &'static [u8] = b"http://example.org/eg-amp_rust\0";
static mut desc: lv2::LV2Descriptor = lv2::LV2Descriptor {
    uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
    instantiate: lv2::instantiate::<Newtype>,
    connect_port: lv2::connect_port::<Newtype>,
    activate: lv2::activate,
    run: lv2::run::<Newtype>,
    deactivate: lv2::deactivate,
    cleanup: lv2::cleanup,
    extension_data: lv2::extension_data,
};


#[no_mangle]
pub extern "C" fn lv2_descriptor(index: i32) -> *const lv2::LV2Descriptor {
    if index != 0 {
        return ptr::null();
    } else {
        // credits to ker on stackoverflow: http://stackoverflow.com/questions/31334356/static-struct-with-c-strings-for-lv2-plugin (duplicate) or http://stackoverflow.com/questions/25880043/creating-a-static-c-struct-containing-strings
        let ptr = S.as_ptr() as *const libc::c_char;
        unsafe {
            desc.uri = ptr;
            return &desc as *const lv2::LV2Descriptor;
        }
    }
}