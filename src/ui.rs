use libc;
use core::*;

pub type LV2UIHandle = *mut libc::c_void;
pub type LV2UIWidget = *mut libc::c_void;
pub type LV2UIController = *const libc::c_void;
pub type LV2UIWriteFunction = extern fn(
	controller: LV2UIController,
	port_index: libc::c_uint,
	buffer_size: libc::c_uint,
	port_protocol: libc::c_uint,
	buffer: *const libc::c_void);
	
#[repr(C)]
pub struct LV2UIIdleInterface {
	pub idle: extern fn(ui: LV2UIHandle) -> libc::c_int
}


#[repr(C)]
pub struct LV2UIDescriptor {
    pub uri: *const  libc::c_char,
    pub instantiate: extern fn(
	    descriptor: *const LV2UIDescriptor,
		plugin_uri: *const libc::c_char,
		bundle_path: *const libc::c_char,
		write_function: LV2UIWriteFunction,
		controller: LV2UIController,
		widget: *mut LV2UIWidget,
		features: *const (*const LV2Feature))-> LV2UIHandle,
	pub cleanup: extern fn(LV2UIHandle),
	pub port_event: extern fn(
		ui: LV2UIHandle,
		port_index: libc::c_uint,
		buffer_size: libc::c_uint,
		format: libc::c_uint,
		buffer: *const libc::c_void
	),
	pub extension_data: extern fn(*const libc::c_char)-> *const libc::c_void
}
