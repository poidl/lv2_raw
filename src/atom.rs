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
use utils::*;


pub static LV2_ATOM_URI: &'static [u8] = b"http://lv2plug.in/ns/ext/atom\0";
pub static LV2_ATOM_PREFIX: &'static [u8] = b"http://lv2plug.in/ns/ext/atom#\0"; 

pub static LV2_MIDI__ACTIVESENSE      : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#ActiveSense\0";
pub static LV2_MIDI__AFTERTOUCH       : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Aftertouch\0";

pub static LV2_ATOM__ATOM          : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Atom\0";
pub static LV2_ATOM__ATOMPORT      : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#AtomPort\0";
pub static LV2_ATOM__BLANK         : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Blank\0";
pub static LV2_ATOM__BOOL          : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Bool\0";
pub static LV2_ATOM__CHUNK         : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Chunk\0";
pub static LV2_ATOM__DOUBLE        : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Double\0";
pub static LV2_ATOM__EVENT         : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Event\0";
pub static LV2_ATOM__FLOAT         : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Float\0";
pub static LV2_ATOM__INT           : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Int\0";
pub static LV2_ATOM__LITERAL       : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Literal\0";
pub static LV2_ATOM__LONG          : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Long\0";
pub static LV2_ATOM__NUMBER        : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Number\0";
pub static LV2_ATOM__OBJECT        : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Object\0";
pub static LV2_ATOM__PATH          : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Path\0";
pub static LV2_ATOM__PROPERTY      : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Property\0";
pub static LV2_ATOM__RESOURCE      : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Resource\0";
pub static LV2_ATOM__SEQUENCE      : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Sequence\0";
pub static LV2_ATOM__SOUND         : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Sound\0";
pub static LV2_ATOM__STRING        : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#String\0";
pub static LV2_ATOM__TUPLE         : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Tuple\0";
pub static LV2_ATOM__URI           : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#URI\0";
pub static LV2_ATOM__URID          : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#URID\0";
pub static LV2_ATOM__VECTOR        : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#Vector\0";
pub static LV2_ATOM__ATOMTRANSFER  : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#atomTransfer\0";
pub static LV2_ATOM__BEATTIME      : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#beatTime\0";
pub static LV2_ATOM__BUFFERTYPE    : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#bufferType\0";
pub static LV2_ATOM__CHILDTYPE     : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#childType\0";
pub static LV2_ATOM__EVENTTRANSFER : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#eventTransfer\0";
pub static LV2_ATOM__FRAMETIME     : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#frameTime\0";
pub static LV2_ATOM__SUPPORTS      : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#supports\0";
pub static LV2_ATOM__TIMEUNIT      : &'static [u8] = b"http://lv2plug.in/ns/ext/atom#timeUnit\0";





/** The header of an atom:Atom. */
#[repr(C)]
pub struct LV2_Atom {
    /**< Size in bytes, not including type and size. */
    pub size: u32,
    /**< Type of this atom (mapped URI). */
    pub mytype: u32,
}

/** An atom:Int or atom:Bool.  May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_Int {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /**< Integer value. */
    pub body: i32
}

/** An atom:Long.  May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_Long {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /**< Integer value. */
    pub body: i64
}

/** An atom:Float.  May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_Float {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /**< Float value. */
    pub body: f32
}

/** An atom:Double.  May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_Double {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /**< Double value. */
    pub body: f64
}

pub type LV2_Atom_Bool = LV2_Atom_Int;

/** An atom:URID.  May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_URID {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /**< URID. */
    pub body: u32
}

/** An atom:String.  May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_String {
    /**< Atom header. */
    pub atom: LV2_Atom
    /* Contents (a null-terminated UTF-8 string) follow here. */
}

/** The body of an atom:Literal. */
#[repr(C)]
pub struct LV2_Atom_Literal_Body {
    /**< Datatype URID. */
    pub datatype: u32,
    /**< Language URID. */
    pub lang: u32
    /* Contents (a null-terminated UTF-8 string) follow here. */
}

/** An atom:Literal.  May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_Literal {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /**< URID. */
    pub body: LV2_Atom_Literal_Body
}

/** An atom:Tuple.  May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_Tuple {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /* Contents (a series of complete atoms) follow here. */
}

/** The body of an atom:Vector. */
#[repr(C)]
pub struct LV2_Atom_Vector_Body {
    /**< The size of each element in the vector. */
    pub child_size: u32,
    /**< The type of each element in the vector. */
    pub child_type: u32
    /* Contents (a series of packed atom bodies) follow here. */
}

/** An atom:Vector.  May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_Vector {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /**< Body. */
    pub body: LV2_Atom_Vector_Body
}

/** The body of an atom:Property (e.g. in an atom:Object). */
#[repr(C)]
pub struct LV2_Atom_Property_Body {
    /**< Key (predicate) (mapped URI). */
    pub key: u32,
    /**< Context URID (may be, and generally is, 0). */
    pub context: u32,
    /**< Value atom header. */
    pub value: LV2_Atom
    /* Value atom body follows here. */
}

/** An atom:Property.  May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_Property {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /**< Body. */
    pub body: LV2_Atom_Property_Body
}

/** The body of an atom:Object. May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_Object_Body {
    /**< URID, or 0 for blank. */
    pub id: u32,
    /**< Type URID (same as rdf:type, for fast dispatch). */
    pub otype: u32
    /* Contents (a series of property bodies) follow here. */
}

/** An atom:Object.  May be cast to LV2_Atom. */
#[repr(C)]
pub struct LV2_Atom_Object {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /**< Body. */
    pub body: LV2_Atom_Object_Body
}


impl LV2_Atom_Object {
 
    pub unsafe fn foreach<F>(&mut self, mut closure: F) -> () 
        where F: FnMut(*mut LV2_Atom_Property_Body) -> bool {

        let body = &(self.body);
        let mut it = lv2_atom_object_begin(body);
        while !lv2_atom_object_is_end(body, self.atom.size, it) {
            let res = closure(it);
            if res { break; }
            it = lv2_atom_object_next(it);
        }
    }
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





