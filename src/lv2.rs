extern crate libc;

pub type Lv2handle = *mut libc::c_void;

// The header of an atom:Atom.
#[repr(C)]
pub struct LV2_Atom {
	size: u32,  // Size in bytes, not including type and size.
	mytype: u32  // Type of this atom (mapped URI).
}

#[repr(C)]
pub struct LV2_Atom_Sequence_Body {
	unit: u32,  // URID of unit of event time stamps.
	pad: u32   // Currently unused.
}

// An atom:Sequence.
#[repr(C)]
pub struct LV2_Atom_Sequence {
	atom: LV2_Atom, // Atom header.
	body: LV2_Atom_Sequence_Body  // Body.
}

pub type LV2_URID_Map_Handle = *mut libc::c_void;
pub type LV2_URID = u32;

#[repr(C)]
pub struct LV2_URID_Map {
	handle: LV2_URID_Map_Handle, // Opaque pointer to host data.
	   //@param handle Must be the callback_data member of this struct.
	   //@param uri The URI to be mapped to an integer ID.
	map: extern fn(handle: LV2_URID_Map_Handle, uri: *const libc::c_char)-> LV2_URID
}

#[repr(C)]
pub struct LV2Feature {
    uri: *const u8,
    data: *mut libc::c_void
}

#[repr(C)]
pub struct LV2Descriptor {
    pub amp_uri: *const  libc::c_char,
    pub instantiate: extern fn(descriptor: *const LV2Descriptor, rate:  &mut f64,
                            bundle_path: *const u8, features: *const LV2Feature)
                                -> Lv2handle,
    pub connect_port: extern fn(handle: Lv2handle, port: u32, data: *mut libc::c_void),
    pub activate: extern fn(instance: Lv2handle),
    pub run: extern fn(instance: Lv2handle, n_samples: u32),
    pub deactivate: extern fn(instance: Lv2handle),
    pub cleanup: extern fn(instance: Lv2handle),
    pub extension_data: extern fn(uri: *const u8)-> (*const libc::c_void),
}
