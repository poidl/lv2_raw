/**
   Return true iff `msg` is a MIDI voice message (which has a channel).
*/
pub fn lv2_midi_is_voice_message(msg: *const u8) -> (bool) {
    unsafe {
        return (*msg) >= 0x80 && (*msg) < 0xF0;
    }
}

/**
   Return the type of a MIDI message.
   @param msg Pointer to the start (status byte) of a MIDI message.
*/
pub fn lv2_midi_message_type(msg: *const u8) -> (Lv2MidiMessageType) {
    if lv2_midi_is_voice_message(msg) {
        unsafe {
            return Lv2MidiMessageType::from_int((*msg) & 0xF0);
        }
    } else {
        return Lv2MidiMessageType::Lv2MidiMsgInvalid;
    }
}

pub enum Lv2MidiMessageType {
    Lv2MidiMsgInvalid = 0, // Invalid Message
    Lv2MidiMsgNoteOff = 0x80, // Note Off
    Lv2MidiMsgNoteOn = 0x90, // Note On
    Lv2MidiMsgNotImplemented = 9999999999999, //
}

// Unnecessary?
impl Lv2MidiMessageType {
    fn from_int(x: u8) -> Lv2MidiMessageType {
        match x {
            0 => Lv2MidiMessageType::Lv2MidiMsgInvalid,
            0x80 => Lv2MidiMessageType::Lv2MidiMsgNoteOff,
            0x90 => Lv2MidiMessageType::Lv2MidiMsgNoteOn,
            _ => Lv2MidiMessageType::Lv2MidiMsgNotImplemented,
        }
    }
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
