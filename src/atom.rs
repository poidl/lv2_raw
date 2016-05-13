use std::mem;

// The header of an atom:Atom.
#[repr(C)]
pub struct LV2_Atom {
	pub size: u32,  // Size in bytes, not including type and size.
	pub mytype: u32  // Type of this atom (mapped uri).
}

// compare with
//http://lv2plug.in/git/cgit.cgi/lv2.git/tree/lv2/lv2plug.in/ns/ext/atom/atom.h
// Lv2AtomEvent has a union "time", which can be beat or frames. Not implemented
// doesn't need #[repr(C)]
pub struct Lv2AtomEvent {
	pub time_in_frames: i64,
	pub body: LV2_Atom
}

#[repr(C)]
pub struct LV2_Atom_Sequence_Body {
	unit: u32,  // uriD of unit of event time stamps.
	pad: u32   // Currently unused.
	/* Contents (a series of events) follow here. */
}

// An atom:Sequence.
#[repr(C)]
pub struct LV2_Atom_Sequence {
	pub atom: LV2_Atom, // Atom header.
	pub body: LV2_Atom_Sequence_Body  // Body.
}

/** Pad a size to 64 bits. */
pub fn lv2_atom_pad_size(size: u32) -> (u32) {
	return (size + 7u32) & (!7u32)
}

/** Get an iterator pointing to the first event in a Sequence body. */
pub fn lv2_atom_sequence_begin(body: *const LV2_Atom_Sequence_Body) ->  (*const Lv2AtomEvent) {
	unsafe{
		return body.offset(1) as *const Lv2AtomEvent
	}
}

/** Return an iterator to the element following `i`. */
pub fn lv2_atom_sequence_next(i: *const Lv2AtomEvent) -> (*const Lv2AtomEvent)
{
	unsafe{
		let addr_of_first_byte = i as *const u8;
		let size_in_bytes_1 = mem::size_of::<Lv2AtomEvent>() as isize;
		let size_in_bytes_2 = lv2_atom_pad_size((*i).body.size) as isize;
		let j = addr_of_first_byte.offset(size_in_bytes_1 + size_in_bytes_2);
		return j as *const Lv2AtomEvent
	}
}

/** Return true iff `i` has reached the end of `body`. */
pub fn lv2_atom_sequence_is_end(body: *const LV2_Atom_Sequence_Body, size: u32, i: *const Lv2AtomEvent) -> (bool) {
	let addr_of_first_byte = body as *const u8;
	unsafe{
		return (i as *const u8) >= addr_of_first_byte.offset(size as isize)
	}
}
