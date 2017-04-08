// Copyright 2017 Stefan Riha, Michael Oswald

// Documentation copied from http://lv2plug.in/ns/ext/urid/urid.h

// Copyright text of the original C file:

// Copyright 2008-2016 David Robillard <http://drobilla.net>
// Copyright 2011 Gabriel M. Beddingfield <gabrbedd@gmail.com>
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
// THIS SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

use libc;

pub type LV2Urid = u32;
pub type LV2UridMapHandle = *mut libc::c_void;

pub static LV2_URID_URI: &'static str = "http://lv2plug.in/ns/ext/urid";
pub static LV2_URID_PREFIX: &'static str = "http://lv2plug.in/ns/ext/urid#";

pub static LV2_URID__MAP: &'static str = "http://lv2plug.in/ns/ext/urid#map";
pub static LV2_URID__UNMAP: &'static str = "http://lv2plug.in/ns/ext/urid#unmap";


/**
   URID Map Feature (LV2_URID__map)
*/
#[repr(C)]
pub struct LV2UridMap {
    /**
	   Opaque pointer to host data.

	   This MUST be passed to map_uri() whenever it is called.
	   Otherwise, it must not be interpreted in any way.
	*/
    pub handle: LV2UridMapHandle,

    /**
	   Get the numeric ID of a URI.

	   If the ID does not already exist, it will be created.

	   This function is referentially transparent; any number of calls with the
	   same arguments is guaranteed to return the same value over the life of a
	   plugin instance.  Note, however, that several URIs MAY resolve to the
	   same ID if the host considers those URIs equivalent.

	   This function is not necessarily very fast or RT-safe: plugins SHOULD
	   cache any IDs they might need in performance critical situations.

	   The return value 0 is reserved and indicates that an ID for that URI
	   could not be created for whatever reason.  However, hosts SHOULD NOT
	   return 0 from this function in non-exceptional circumstances (i.e. the
	   URI map SHOULD be dynamic).

	   @param handle Must be the callback_data member of this struct.
	   @param uri The URI to be mapped to an integer ID.
	*/
    pub map: extern "C" fn(handle: LV2UridMapHandle, uri: *const libc::c_char) -> LV2Urid,
}
