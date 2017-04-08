
use libc::{c_char, c_void};
use core::*;
use std::ffi::*;


/**
   Return the data for a feature in a features array.

   If the feature is not found, NULL is returned.  Note that this function is
   only useful for features with data, and can not detect features that are
   present but have NULL data.
*/
pub unsafe fn lv2_features_data(features: *const *const LV2Feature, curi: *const c_char) -> *mut c_void {
    
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
    required: bool
}

pub unsafe fn lv2_features_query(features: *const *const LV2Feature, query: &[FeatureHelper]) -> *const c_char {

    for it in query {
        let mut data = it.data;
        *data = lv2_features_data(features, it.urid);
        if it.required && (*data).is_null() {
            return it.urid;
        }
    }

    return 0 as *const c_char;
}
