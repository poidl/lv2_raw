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

/// An atom:Sequence.
#[repr(C)]
pub struct LV2_Atom_Sequence {
    /**< Atom header. */
    pub atom: LV2_Atom,
    /**< Body. */
    pub body: LV2_Atom_Sequence_Body,
}
