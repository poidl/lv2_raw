extern crate libc;
use std::mem;

pub type Lv2handle = *mut libc::c_void;


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
	// println!("7u32     = {:0>32b}", 7u32);
	// println!("!7u32     = {:0>32b}", !7u32);
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

pub type Lv2uridMapHandle = *mut libc::c_void;
pub type Lv2urid = u32;

#[repr(C)]
pub struct Lv2uridMap {
	pub handle: Lv2uridMapHandle, // Opaque pointer to host data.
	   //@param handle Must be the callback_data member of this struct.
	   //@param uri The uri to be mapped to an integer ID.
	pub map: extern fn(handle: Lv2uridMapHandle, uri: *const libc::c_char)-> Lv2urid
}

#[repr(C)]
pub struct LV2Feature {
    pub uri: *const libc::c_char,
    pub data: *mut Lv2uridMap
}

#[repr(C)]
pub struct LV2Descriptor {
    pub uri: *const  libc::c_char,
    pub instantiate: extern fn(descriptor: *const LV2Descriptor,rate: f64, bundle_path: *const libc::c_char, 	features: *const (*const LV2Feature) )
                                -> Lv2handle,
    pub connect_port: extern fn(handle: Lv2handle, port: u32, data: *mut libc::c_void),
    pub activate: extern fn(instance: Lv2handle),
    pub run: extern fn(instance: Lv2handle, n_samples: u32),
    pub deactivate: extern fn(instance: Lv2handle),
    pub cleanup: extern fn(instance: Lv2handle),
    pub extension_data: extern fn(uri: *const u8)-> (*const libc::c_void),
}

// typedef enum {
// 	Lv2MidiMsgInvalid          = 0,     /**< Invalid Message */
// 	Lv2MidiMsgNoteOff         = 0x80,  /**< Note Off */
// 	Lv2MidiMsgNoteOn          = 0x90,  /**< Note On */
// 	LV2_MIDI_MSG_NOTE_PRESSURE    = 0xA0,  /**< Note Pressure */
// 	LV2_MIDI_MSG_CONTROLLER       = 0xB0,  /**< Controller */
// 	LV2_MIDI_MSG_PGM_CHANGE       = 0xC0,  /**< Program Change */
// 	LV2_MIDI_MSG_CHANNEL_PRESSURE = 0xD0,  /**< Channel Pressure */
// 	LV2_MIDI_MSG_BENDER           = 0xE0,  /**< Pitch Bender */
// 	LV2_MIDI_MSG_SYSTEM_EXCLUSIVE = 0xF0,  /**< System Exclusive Begin */
// 	LV2_MIDI_MSG_MTC_QUARTER      = 0xF1,  /**< MTC Quarter Frame */
// 	LV2_MIDI_MSG_SONG_POS         = 0xF2,  /**< Song Position */
// 	LV2_MIDI_MSG_SONG_SELECT      = 0xF3,  /**< Song Select */
// 	LV2_MIDI_MSG_TUNE_REQUEST     = 0xF6,  /**< Tune Request */
// 	LV2_MIDI_MSG_CLOCK            = 0xF8,  /**< Clock */
// 	LV2_MIDI_MSG_START            = 0xFA,  /**< Start */
// 	LV2_MIDI_MSG_CONTINUE         = 0xFB,  /**< Continue */
// 	LV2_MIDI_MSG_STOP             = 0xFC,  /**< Stop */
// 	LV2_MIDI_MSG_ACTIVE_SENSE     = 0xFE,  /**< Active Sensing */
// 	LV2_MIDI_MSG_RESET            = 0xFF   /**< Reset */
// } Lv2MidiMessageType;

/**
   Return true iff `msg` is a MIDI voice message (which has a channel).
*/
pub fn lv2_midi_is_voice_message(msg: *const u8) -> (bool) {
	unsafe{
		return (*msg) >= 0x80 && (*msg) < 0xF0;
	}
}

/**
   Return the type of a MIDI message.
   @param msg Pointer to the start (status byte) of a MIDI message.
*/
pub fn lv2_midi_message_type(msg: *const u8) -> (Lv2MidiMessageType) {
	if lv2_midi_is_voice_message(msg) {
		unsafe{
			return Lv2MidiMessageType::from_int((*msg) & 0xF0);
		}
	// } else if (lv2_midi_is_system_message(msg)) {
	// 	return (Lv2MidiMessageType)msg[0];
	} else {
		return Lv2MidiMessageType::Lv2MidiMsgInvalid;
	}
}

pub enum Lv2MidiMessageType {
	Lv2MidiMsgInvalid          = 0,    // Invalid Message
	Lv2MidiMsgNoteOff         = 0x80, // Note Off
	Lv2MidiMsgNoteOn          = 0x90,  // Note On
	Lv2MidiMsgNotImplemented  = 9999999999999  //
}

// Unnecessary?
impl Lv2MidiMessageType {
    fn from_int(x: u8) -> Lv2MidiMessageType {
        match x {
            0 => Lv2MidiMessageType::Lv2MidiMsgInvalid,
            0x80 => Lv2MidiMessageType::Lv2MidiMsgNoteOff,
			0x90 => Lv2MidiMessageType::Lv2MidiMsgNoteOn,
            _ => Lv2MidiMessageType::Lv2MidiMsgNotImplemented
        }
    }
}
