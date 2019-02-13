// Copyright 2017 Michael Oswald

// Documentation copied from http://lv2plug.in/ns/ext/midi/midi.h

// Copyright text of the original C file:

// Copyright 2012-2016 David Robillard <http://drobilla.net>
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

//! Documentation of the corresponding C header files: http://lv2plug.in/ns/ext/midi/midi.html.

pub static LV2_MIDI_URI: &'static [u8] = b"http://lv2plug.in/ns/ext/midi\0";
pub static LV2_MIDI_PREFIX: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#\0";

pub static LV2_MIDI__ACTIVESENSE: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#ActiveSense\0";
pub static LV2_MIDI__AFTERTOUCH: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Aftertouch\0";
pub static LV2_MIDI__BENDER: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Bender\0";
pub static LV2_MIDI__CHANNELPRESSURE: &'static [u8] =
    b"http://lv2plug.in/ns/ext/midi#ChannelPressure\0";
pub static LV2_MIDI__CHUNK: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Chunk\0";
pub static LV2_MIDI__CLOCK: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Clock\0";
pub static LV2_MIDI__CONTINUE: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Continue\0";
pub static LV2_MIDI__CONTROLLER: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Controller\0";
pub static LV2_MIDI__MIDIEVENT: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#MidiEvent\0";
pub static LV2_MIDI__NOTEOFF: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#NoteOff\0";
pub static LV2_MIDI__NOTEON: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#NoteOn\0";
pub static LV2_MIDI__PROGRAMCHANGE: &'static [u8] =
    b"http://lv2plug.in/ns/ext/midi#ProgramChange\0";
pub static LV2_MIDI__QUARTERFRAME: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#QuarterFrame\0";
pub static LV2_MIDI__RESET: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Reset\0";
pub static LV2_MIDI__SONGPOSITION: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#SongPosition\0";
pub static LV2_MIDI__SONGSELECT: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#SongSelect\0";
pub static LV2_MIDI__START: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Start\0";
pub static LV2_MIDI__STOP: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Stop\0";
pub static LV2_MIDI__SYSTEMCOMMON: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#SystemCommon\0";
pub static LV2_MIDI__SYSTEMEXCLUSIVE: &'static [u8] =
    b"http://lv2plug.in/ns/ext/midi#SystemExclusive\0";
pub static LV2_MIDI__SYSTEMMESSAGE: &'static [u8] =
    b"http://lv2plug.in/ns/ext/midi#SystemMessage\0";
pub static LV2_MIDI__SYSTEMREALTIME: &'static [u8] =
    b"http://lv2plug.in/ns/ext/midi#SystemRealtime\0";
pub static LV2_MIDI__TICK: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Tick\0";
pub static LV2_MIDI__TUNEREQUEST: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#TuneRequest\0";
pub static LV2_MIDI__VOICEMESSAGE: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#VoiceMessage\0";
pub static LV2_MIDI__BENDERVALUE: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#benderValue\0";
pub static LV2_MIDI__BINDING: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#binding\0";
pub static LV2_MIDI__BYTENUMBER: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#byteNumber\0";
pub static LV2_MIDI__CHANNEL: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#channel\0";
pub static LV2_MIDI___CHUNK: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#chunk\0";
pub static LV2_MIDI__CONTROLLERNUMBER: &'static [u8] =
    b"http://lv2plug.in/ns/ext/midi#controllerNumber\0";
pub static LV2_MIDI__CONTROLLERVALUE: &'static [u8] =
    b"http://lv2plug.in/ns/ext/midi#controllerValue\0";
pub static LV2_MIDI__NOTENUMBER: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#noteNumber\0";
pub static LV2_MIDI__PRESSURE: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#pressure\0";
pub static LV2_MIDI__PROGRAMNUMBER: &'static [u8] =
    b"http://lv2plug.in/ns/ext/midi#programNumber\0";
pub static LV2_MIDI__PROPERTY: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#property\0";
pub static LV2_MIDI__SONGNUMBER: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#songNumber\0";
pub static LV2_MIDI___SONGPOSITION: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#songPosition\0";
pub static LV2_MIDI__STATUS: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#status\0";
pub static LV2_MIDI__STATUSMASK: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#statusMask\0";
pub static LV2_MIDI__VELOCITY: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#velocity\0";

/**
   MIDI Message Type.

   This includes both voice messages (which have a channel) and system messages
   (which do not), as well as a sentinel value for invalid messages.  To get
   the type of a message suitable for use in a switch statement, use
   lv2_midi_get_type() on the status byte.
*/
pub enum LV2MidiMessageType {
    LV2MidiMsgInvalid,
    LV2MidiMsgNoteOff,
    LV2MidiMsgNoteOn,
    LV2MidiMsgNotePressure,
    LV2MidiMsgController,
    LV2MidiMsgPgmChange,
    LV2MidiMsgChannelPressure,
    LV2MidiMsgBender,
    LV2MidiMsgSystemExclusive,
    LV2MidiMsgMtcQuarter,
    LV2MidiMsgSongPos,
    LV2MidiMsgSongSelect,
    LV2MidiMsgTuneRequest,
    LV2MidiMsgClock,
    LV2MidiMsgStart,
    LV2MidiMsgContinue,
    LV2MidiMsgStop,
    LV2MidiMsgActiveSense,
    LV2MidiMsgReset,
}

impl LV2MidiMessageType {
    pub fn from_u8(x: u8) -> LV2MidiMessageType {
        match x {
            0 => LV2MidiMessageType::LV2MidiMsgInvalid,
            0x80 => LV2MidiMessageType::LV2MidiMsgNoteOff,
            0x90 => LV2MidiMessageType::LV2MidiMsgNoteOn,
            0xA0 => LV2MidiMessageType::LV2MidiMsgNotePressure,
            0xB0 => LV2MidiMessageType::LV2MidiMsgController,
            0xC0 => LV2MidiMessageType::LV2MidiMsgPgmChange,
            0xD0 => LV2MidiMessageType::LV2MidiMsgChannelPressure,
            0xE0 => LV2MidiMessageType::LV2MidiMsgBender,
            0xF0 => LV2MidiMessageType::LV2MidiMsgSystemExclusive,
            0xF1 => LV2MidiMessageType::LV2MidiMsgMtcQuarter,
            0xF2 => LV2MidiMessageType::LV2MidiMsgSongPos,
            0xF3 => LV2MidiMessageType::LV2MidiMsgSongSelect,
            0xF6 => LV2MidiMessageType::LV2MidiMsgTuneRequest,
            0xF8 => LV2MidiMessageType::LV2MidiMsgClock,
            0xFA => LV2MidiMessageType::LV2MidiMsgStart,
            0xFB => LV2MidiMessageType::LV2MidiMsgContinue,
            0xFC => LV2MidiMessageType::LV2MidiMsgStop,
            0xFE => LV2MidiMessageType::LV2MidiMsgActiveSense,
            0xFF => LV2MidiMessageType::LV2MidiMsgReset,
            _ => LV2MidiMessageType::LV2MidiMsgInvalid,
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            LV2MidiMessageType::LV2MidiMsgInvalid => 0,
            LV2MidiMessageType::LV2MidiMsgNoteOff => 0x80,
            LV2MidiMessageType::LV2MidiMsgNoteOn => 0x90,
            LV2MidiMessageType::LV2MidiMsgNotePressure => 0xA0,
            LV2MidiMessageType::LV2MidiMsgController => 0xB0,
            LV2MidiMessageType::LV2MidiMsgPgmChange => 0xC0,
            LV2MidiMessageType::LV2MidiMsgChannelPressure => 0xD0,
            LV2MidiMessageType::LV2MidiMsgBender => 0xE0,
            LV2MidiMessageType::LV2MidiMsgSystemExclusive => 0xF0,
            LV2MidiMessageType::LV2MidiMsgMtcQuarter => 0xF1,
            LV2MidiMessageType::LV2MidiMsgSongPos => 0xF2,
            LV2MidiMessageType::LV2MidiMsgSongSelect => 0xF3,
            LV2MidiMessageType::LV2MidiMsgTuneRequest => 0xF6,
            LV2MidiMessageType::LV2MidiMsgClock => 0xF8,
            LV2MidiMessageType::LV2MidiMsgStart => 0xFA,
            LV2MidiMessageType::LV2MidiMsgContinue => 0xFB,
            LV2MidiMessageType::LV2MidiMsgStop => 0xFC,
            LV2MidiMessageType::LV2MidiMsgActiveSense => 0xFE,
            LV2MidiMessageType::LV2MidiMsgReset => 0xFF,
        }
    }
}

/**
   Standard MIDI Controller Numbers.
*/
pub enum LV2MidiController {
    LV2MidiCtlMsbBank = 0x00,
    LV2MidiCtlMsbModwheel = 0x01,
    LV2MidiCtlMsbBreath = 0x02,
    LV2MidiCtlMsbFoot = 0x04,
    LV2MidiCtlMsbPortamentoTime = 0x05,
    LV2MidiCtlMsbDataEntry = 0x06,
    LV2MidiCtlMsbMainVolume = 0x07,
    LV2MidiCtlMsbBalance = 0x08,
    LV2MidiCtlMsbPan = 0x0A,
    LV2MidiCtlMsbExpression = 0x0B,
    LV2MidiCtlMsbEffect1 = 0x0C,
    LV2MidiCtlMsbEffect2 = 0x0D,
    LV2MidiCtlMsbGeneralPurpose1 = 0x10,
    LV2MidiCtlMsbGeneralPurpose2 = 0x11,
    LV2MidiCtlMsbGeneralPurpose3 = 0x12,
    LV2MidiCtlMsbGeneralPurpose4 = 0x13,
    LV2MidiCtlLsbBank = 0x20,
    LV2MidiCtlLsbModwheel = 0x21,
    LV2MidiCtlLsbBreath = 0x22,
    LV2MidiCtlLsbFoot = 0x24,
    LV2MidiCtlLsbPortamentoTime = 0x25,
    LV2MidiCtlLsbDataEtry = 0x26,
    LV2MidiCtlLsbMainVolume = 0x27,
    LV2MidiCtlLsbBalance = 0x28,
    LV2MidiCtlLsbPan = 0x2A,
    LV2MidiCtlLsbExpression = 0x2B,
    LV2MidiCtlLsbEffect1 = 0x2C,
    LV2MidiCtlLsbEffect2 = 0x2D,
    LV2MidiCtlLsbGeneralPurpose1 = 0x30,
    LV2MidiCtlLsbGeneralPurpose2 = 0x31,
    LV2MidiCtlLsbGeneralPurpose3 = 0x32,
    LV2MidiCtlLsbGeneralPurpose4 = 0x33,
    LV2MidiCtlSustain = 0x40,
    LV2MidiCtlPortamento = 0x41,
    LV2MidiCtlSostenuto = 0x42,
    LV2MidiCtlSoftPedal = 0x43,
    LV2MidiCtlLegatoFootswitch = 0x44,
    LV2MidiCtlHold2 = 0x45,
    LV2MidiCtlSc1SoundVariation = 0x46,
    LV2MidiCtlSc2Timbre = 0x47,
    LV2MidiCtlSc3ReleaseTime = 0x48,
    LV2MidiCtlSc4AttackTime = 0x49,
    LV2MidiCtlSc5Brightness = 0x4A,
    LV2MidiCtlSc6 = 0x4B,
    LV2MidiCtlSc7 = 0x4C,
    LV2MidiCtlSc8 = 0x4D,
    LV2MidiCtlSc9 = 0x4E,
    LV2MidiCtlSc10 = 0x4F,
    LV2MidiCtlGeneralPurpose5 = 0x50,
    LV2MidiCtlGeneralPurpose6 = 0x51,
    LV2MidiCtlGeneralPurpose7 = 0x52,
    LV2MidiCtlGeneralPurpose8 = 0x53,
    LV2MidiCtlPortamentoControl = 0x54,
    LV2MidiCtlE1ReverbDepth = 0x5B,
    LV2MidiCtlE2TremoloDepth = 0x5C,
    LV2MidiCtlE3ChorusDepth = 0x5D,
    LV2MidiCtlE4DetuneDepth = 0x5E,
    LV2MidiCtlE5PhaserDepth = 0x5F,
    LV2MidiCtlDataIncrement = 0x60,
    LV2MidiCtlDataDecrement = 0x61,
    LV2MidiCtlNrpnLsb = 0x62,
    LV2MidiCtlNrpnMsb = 0x63,
    LV2MidiCtlRpnLsb = 0x64,
    LV2MidiCtlRpnMsb = 0x65,
    LV2MidiCtlAllSoundsOff = 0x78,
    LV2MidiCtlResetControllers = 0x79,
    LV2MidiCtlLocalControlSwitch = 0x7A,
    LV2MidiCtlAllNotesOff = 0x7B,
    LV2MidiCtlOmniOff = 0x7C,
    LV2MidiCtlOmniOn = 0x7D,
    LV2MidiCtlMono1 = 0x7E,
    LV2MidiCtlMono2 = 0x7F,
}

/**
   Return true iff `msg` is a MIDI voice message (which has a channel).
*/
pub fn lv2_midi_is_voice_message(msg: &[u8]) -> bool {
    msg[0] >= 0x80 && msg[0] < 0xF0
}

/**
   Return true iff `msg` is a MIDI system message (which has no channel).
*/
pub fn lv2_midi_is_system_message(msg: &[u8]) -> bool {
    match msg[0] {
        0xF4 => false,
        0xF5 => false,
        0xF7 => false,
        0xF9 => false,
        0xFD => false,
        _ => true,
    }
}

/**
   Return the type of a MIDI message.
   @param msg Pointer to the start (status byte) of a MIDI message.
*/
pub fn lv2_midi_message_type(msg: &[u8]) -> LV2MidiMessageType {
    if lv2_midi_is_voice_message(msg) {
        LV2MidiMessageType::from_u8(msg[0] & 0xF0)
    } else if lv2_midi_is_system_message(msg) {
        LV2MidiMessageType::from_u8(msg[0])
    } else {
        LV2MidiMessageType::LV2MidiMsgInvalid
    }
}
