// Copyright 2017 Stefan Riha

// Documentation copied from http://lv2plug.in/ns/ext/atom/atom.h

// Copyright text of the original C file:

// Copyright 2008-2016 David Robillard <http://drobilla.net>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THIS SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

use std::mem::{size_of, transmute};
use libc::{memcmp, c_void};

/** The header of an atom:Atom. */
#[repr(C)]
pub struct LV2_Atom {
    /**< Size in bytes, not including type and size. */
    pub size: u32,
    /**< Type of this atom (mapped URI). */
    pub mytype: u32,
}

/** The header of an atom:Event.  Note this type is NOT an LV2_Atom. */
#[repr(C)]
pub struct Lv2AtomEvent {
    /** RUST_TODO: inconsistent with the C version, see http://lv2plug.in/git/cgit.cgi/lv2.git/tree/lv2/lv2plug in/ns/ext/atom/atom.h Lv2AtomEvent has a union "time", which can be beat or frames. Not implemented. */
    pub time_in_frames: i64,
    /**< Event body atom header. */
    pub body: LV2_Atom,
}

impl Lv2AtomEvent {
    pub fn time_as_frames(&self) -> i64 {
        self.time_in_frames
    }

    pub fn time_as_beats(&self) -> f64 {
        unsafe { transmute::<i64, f64>(self.time_in_frames) }
    }
}

/**
   The body of an atom:Sequence (a sequence of events).

   The unit field is either a URID that described an appropriate time stamp
   type, or may be 0 where a default stamp type is known.  For
   LV2_Descriptor::run(), the default stamp type is audio frames.

   The contents of a sequence is a series of LV2_Atom_Event, each aligned
   to 64-bits, e.g.:
   <pre>
   | Event 1 (size 6)                              | Event 2
   |       |       |       |       |       |       |       |       |
   | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | |
   |FRAMES |SUBFRMS|TYPE   |SIZE   |DATADATADATAPAD|FRAMES |SUBFRMS|...
   </pre>
*/
#[repr(C)]
pub struct LV2_Atom_Sequence_Body {
    /**< URID of unit of event time stamps. */
    unit: u32,
    /**< Currently unused. */
    pad: u32, // Contents (a series of events) follow here.
}

impl LV2_Atom_Sequence_Body {
    pub unsafe fn foreach<F>(&mut self, size: u32, mut closure: F) -> () 
        where F: FnMut(*const Lv2AtomEvent) -> () {

        let mut it = lv2_atom_sequence_begin(self);
        while !lv2_atom_sequence_is_end(self, size, it) {
            closure(it);
            it = lv2_atom_sequence_next(it);
        }
    }
}

/// An atom:Sequence.
#[repr(C)]
pub struct LV2_Atom_Sequence {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /**< Body. */
    pub body: LV2_Atom_Sequence_Body,
}


impl LV2_Atom_Sequence {
 
    pub unsafe fn foreach<F>(&mut self, mut closure: F) -> () 
        where F: FnMut(*const Lv2AtomEvent) -> () {

        let body = &(self.body);
        let mut it = lv2_atom_sequence_begin(body);
        while !lv2_atom_sequence_is_end(body, self.atom.size, it) {
            closure(it);
            it = lv2_atom_sequence_next(it);
        }
    }
}






/// Pad a size to 64 bits
pub fn lv2_atom_pad_size(size: u32) -> u32 {
    (size + 7) & (!7)
}

/** Return the total size of `atom`, including the header. */
pub fn lv2_atom_total_size(atom: &LV2_Atom) -> u32 {
    size_of::<LV2_Atom>() as u32 + atom.size
}

/** Return true iff `atom` is null. */
pub unsafe fn lv2_atom_is_null(atom: *const LV2_Atom) -> bool {
    atom.is_null() || ((*atom).mytype == 0 && (*atom).size == 0)
}

/** Return true iff `a` is equal to `b`. */
pub unsafe fn lv2_atom_equals(a: *const LV2_Atom, b: *const LV2_Atom) -> bool {
    (a == b) || (((*a).mytype == (*b).mytype) &&
                 ((*a).size == (*b).size) &&
                 (memcmp(a.offset(1) as *const c_void, 
                         b.offset(1) as *const c_void, 
                         (*a).size as usize) == 0))
}


/** Get an iterator pointing to the first event in a Sequence body. */
pub unsafe fn lv2_atom_sequence_begin(body: *const LV2_Atom_Sequence_Body) -> *const Lv2AtomEvent {
    body.offset(1) as *const Lv2AtomEvent
}

/** Get an iterator pointing to the end of a Sequence body. */
pub unsafe fn lv2_atom_sequence_end(body: *const LV2_Atom_Sequence_Body,
                                    size: u32) -> *const Lv2AtomEvent {

    (body as *const u8).offset(lv2_atom_pad_size(size) as isize) as *const Lv2AtomEvent
}

/** Return true iff `i` has reached the end of `body`. */
pub unsafe fn lv2_atom_sequence_is_end(body: *const LV2_Atom_Sequence_Body,
            size: u32, i: *const Lv2AtomEvent) -> bool {

    let result = i as *const u8 >= (body as *const u8).offset(size as isize);
    println!("lv2_atom_sequence_is_end: {}", result);
    result
}


/** Return an iterator to the element following `i`. */
pub unsafe fn lv2_atom_sequence_next(i: *const Lv2AtomEvent) -> *const Lv2AtomEvent {
    let off = size_of::<Lv2AtomEvent>() + lv2_atom_pad_size((*i).body.size) as usize;
    let ptr = (i as *const u8).offset(off as isize);

    println!("lv2_atom_sequence_next: off: {} ptr: {:?}", off, ptr);

    ptr as *const Lv2AtomEvent
}

/**
   Clear all events from `sequence`.

   This simply resets the size field, the other fields are left untouched.
*/
pub unsafe fn lv2_atom_sequence_clear(seq: *mut LV2_Atom_Sequence) -> () {
    (*seq).atom.size = size_of::<LV2_Atom_Sequence_Body>() as u32;
}

