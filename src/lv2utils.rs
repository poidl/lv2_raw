
use libc::{c_char, c_void};
use core::*;
use std::ffi::*;


/**
   Return the data for a feature in a features array.

   If the feature is not found, NULL is returned.  Note that this function is
   only useful for features with data, and can not detect features that are
   present but have NULL data.
*/
pub unsafe fn lv2_features_data(features: *const *const LV2Feature, curi: *const c_char) -> *const c_void {
    
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
    0 as *const c_void
}



