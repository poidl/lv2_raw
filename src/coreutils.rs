// Copyright 2017 Michael Oswald

// Documentation copied from http://lv2plug.in/ns/lv2core/lv2_util.h

// Copyright text of the original C file:

// Copyright 2016 David Robillard <http://drobilla.net>

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

//! Documentation of the corresponding C header files (part of LV2 core): http://lv2plug.in/ns/lv2core/.

use libc::{c_char, c_void};
use core::*;
use std::ffi::*;


/**
   Return the data for a feature in a features array.

   If the feature is not found, NULL is returned.  Note that this function is
   only useful for features with data, and can not detect features that are
   present but have NULL data.
*/
pub unsafe fn lv2_features_data(features: *const *const LV2Feature,
                                curi: *const c_char)
                                -> *mut c_void {

    if !features.is_null() {
        let mut feature = *features;
        let nul = 0 as *const LV2Feature;
        let uri = CStr::from_ptr(curi).to_string_lossy().into_owned();

        let mut i = 0;
        while feature != nul {
            let f = CStr::from_ptr((*feature).uri).to_string_lossy().into_owned();
            if f == uri {
                return (*feature).data;
            }

            feature = *features.offset(i);
            i += 1;
        }

    }
    0 as *mut c_void
}


pub struct FeatureHelper {
    urid: *const c_char,
    data: *mut *mut c_void,
    required: bool,
}


/**
   Query a features array.

   This function allows getting several features in one call, and detect
   missing required features, with the same caveat of lv2_features_data().

   The arguments should be a series of const char* uri, void** data, bool
   required, terminated by a NULL URI.  The data pointers MUST be initialized
   to NULL.  For example:

   @code
   LV2_URID_Log* log = NULL;
   LV2_URID_Map* map = NULL;
   const char* missing = lv2_features_query(
        features,
        LV2_LOG__log,  &log, false,
        LV2_URID__map, &map, true,
        NULL);
   @endcode

   @return NULL on success, otherwise the URI of this missing feature.
*/
pub unsafe fn lv2_features_query(features: *const *const LV2Feature,
                                 query: &[FeatureHelper])
                                 -> *const c_char {

    for it in query {
        let mut data = it.data;
        *data = lv2_features_data(features, it.urid);
        if it.required && (*data).is_null() {
            return it.urid;
        }
    }

    return 0 as *const c_char;
}
