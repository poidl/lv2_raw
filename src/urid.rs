use libc;

pub type LV2Urid = u32;
pub type LV2UridMapHandle = *mut libc::c_void;

#[repr(C)]
pub struct LV2UridMap {
	pub handle: LV2UridMapHandle, // Opaque pointer to host data.
	   //@param handle Must be the callback_data member of this struct.
	   //@param uri The uri to be mapped to an integer ID.
	pub map: extern fn(handle: LV2UridMapHandle, uri: *const libc::c_char)-> LV2Urid
}
