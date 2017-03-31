
use libc;

#[allow(non_upper_case_globals)]


pub static LV2_MIDI_URI: &'static [u8] = b"http://lv2plug.in/ns/ext/midi\0";
pub static LV2_MIDI_PREFIX: &'static [u8] = b"http://lv2plug.in/ns/ext/midi#\0"; 

pub static LV2_MIDI__ACTIVESENSE      : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#ActiveSense\0";
pub static LV2_MIDI__AFTERTOUCH       : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Aftertouch\0";
pub static LV2_MIDI__BENDER           : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Bender\0";
pub static LV2_MIDI__CHANNELPRESSURE  : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#ChannelPressure\0";
pub static LV2_MIDI__CHUNK            : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Chunk\0";
pub static LV2_MIDI__CLOCK            : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Clock\0";
pub static LV2_MIDI__CONTINUE         : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Continue\0";
pub static LV2_MIDI__CONTROLLER       : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Controller\0";
pub static LV2_MIDI__MIDIEVENT        : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#MidiEvent\0";
pub static LV2_MIDI__NOTEOFF          : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#NoteOff\0";
pub static LV2_MIDI__NOTEON           : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#NoteOn\0";
pub static LV2_MIDI__PROGRAMCHANGE    : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#ProgramChange\0";
pub static LV2_MIDI__QUARTERFRAME     : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#QuarterFrame\0";
pub static LV2_MIDI__RESET            : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Reset\0";
pub static LV2_MIDI__SONGPOSITION     : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#SongPosition\0";
pub static LV2_MIDI__SONGSELECT       : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#SongSelect\0";
pub static LV2_MIDI__START            : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Start\0";
pub static LV2_MIDI__STOP             : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Stop\0";
pub static LV2_MIDI__SYSTEMCOMMON     : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#SystemCommon\0";
pub static LV2_MIDI__SYSTEMEXCLUSIVE  : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#SystemExclusive\0";
pub static LV2_MIDI__SYSTEMMESSAGE    : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#SystemMessage\0";
pub static LV2_MIDI__SYSTEMREALTIME   : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#SystemRealtime\0";
pub static LV2_MIDI__TICK             : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#Tick\0";
pub static LV2_MIDI__TUNEREQUEST      : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#TuneRequest\0";
pub static LV2_MIDI__VOICEMESSAGE     : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#VoiceMessage\0";
pub static LV2_MIDI__BENDERVALUE      : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#benderValue\0";
pub static LV2_MIDI__BINDING          : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#binding\0";
pub static LV2_MIDI__BYTENUMBER       : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#byteNumber\0";
pub static LV2_MIDI__CHANNEL          : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#channel\0";
pub static LV2_MIDI___CHUNK           : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#chunk\0";
pub static LV2_MIDI__CONTROLLERNUMBER : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#controllerNumber\0";
pub static LV2_MIDI__CONTROLLERVALUE  : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#controllerValue\0";
pub static LV2_MIDI__NOTENUMBER       : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#noteNumber\0";
pub static LV2_MIDI__PRESSURE         : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#pressure\0";
pub static LV2_MIDI__PROGRAMNUMBER    : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#programNumber\0";
pub static LV2_MIDI__PROPERTY         : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#property\0";
pub static LV2_MIDI__SONGNUMBER       : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#songNumber\0";
pub static LV2_MIDI___SONGPOSITION    : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#songPosition\0";
pub static LV2_MIDI__STATUS           : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#status\0";
pub static LV2_MIDI__STATUSMASK       : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#statusMask\0";
pub static LV2_MIDI__VELOCITY         : &'static [u8] = b"http://lv2plug.in/ns/ext/midi#velocity\0";



#[allow(non_camel_case_types)]
pub enum LV2_Midi_Message_Type {
	LV2_MIDI_MSG_INVALID          ,  
	LV2_MIDI_MSG_NOTE_OFF         ,  
	LV2_MIDI_MSG_NOTE_ON          ,  
	LV2_MIDI_MSG_NOTE_PRESSURE    ,  
	LV2_MIDI_MSG_CONTROLLER       ,  
	LV2_MIDI_MSG_PGM_CHANGE       ,  
	LV2_MIDI_MSG_CHANNEL_PRESSURE ,  
	LV2_MIDI_MSG_BENDER           ,  
	LV2_MIDI_MSG_SYSTEM_EXCLUSIVE ,  
	LV2_MIDI_MSG_MTC_QUARTER      ,  
	LV2_MIDI_MSG_SONG_POS         ,  
	LV2_MIDI_MSG_SONG_SELECT      ,  
	LV2_MIDI_MSG_TUNE_REQUEST     ,  
	LV2_MIDI_MSG_CLOCK            ,  
	LV2_MIDI_MSG_START            ,  
	LV2_MIDI_MSG_CONTINUE         ,  
	LV2_MIDI_MSG_STOP             ,  
	LV2_MIDI_MSG_ACTIVE_SENSE     ,  
	LV2_MIDI_MSG_RESET            
}


impl LV2_Midi_Message_Type {

    pub fn from_u8(x: u8) -> LV2_Midi_Message_Type {
        match x {
			0    => LV2_Midi_Message_Type::LV2_MIDI_MSG_INVALID          ,
			0x80 => LV2_Midi_Message_Type::LV2_MIDI_MSG_NOTE_OFF         ,
			0x90 => LV2_Midi_Message_Type::LV2_MIDI_MSG_NOTE_ON          ,
			0xA0 => LV2_Midi_Message_Type::LV2_MIDI_MSG_NOTE_PRESSURE    ,
			0xB0 => LV2_Midi_Message_Type::LV2_MIDI_MSG_CONTROLLER       ,
			0xC0 => LV2_Midi_Message_Type::LV2_MIDI_MSG_PGM_CHANGE       ,
			0xD0 => LV2_Midi_Message_Type::LV2_MIDI_MSG_CHANNEL_PRESSURE ,
			0xE0 => LV2_Midi_Message_Type::LV2_MIDI_MSG_BENDER           ,
			0xF0 => LV2_Midi_Message_Type::LV2_MIDI_MSG_SYSTEM_EXCLUSIVE ,
			0xF1 => LV2_Midi_Message_Type::LV2_MIDI_MSG_MTC_QUARTER      ,
			0xF2 => LV2_Midi_Message_Type::LV2_MIDI_MSG_SONG_POS         ,
			0xF3 => LV2_Midi_Message_Type::LV2_MIDI_MSG_SONG_SELECT      ,
			0xF6 => LV2_Midi_Message_Type::LV2_MIDI_MSG_TUNE_REQUEST     ,
			0xF8 => LV2_Midi_Message_Type::LV2_MIDI_MSG_CLOCK            ,
			0xFA => LV2_Midi_Message_Type::LV2_MIDI_MSG_START            ,
			0xFB => LV2_Midi_Message_Type::LV2_MIDI_MSG_CONTINUE         ,
			0xFC => LV2_Midi_Message_Type::LV2_MIDI_MSG_STOP             ,
			0xFE => LV2_Midi_Message_Type::LV2_MIDI_MSG_ACTIVE_SENSE     ,
			0xFF => LV2_Midi_Message_Type::LV2_MIDI_MSG_RESET            ,
			_ => LV2_Midi_Message_Type::LV2_MIDI_MSG_INVALID
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
			LV2_Midi_Message_Type::LV2_MIDI_MSG_INVALID          => 0,   
			LV2_Midi_Message_Type::LV2_MIDI_MSG_NOTE_OFF         => 0x80,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_NOTE_ON          => 0x90,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_NOTE_PRESSURE    => 0xA0,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_CONTROLLER       => 0xB0,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_PGM_CHANGE       => 0xC0,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_CHANNEL_PRESSURE => 0xD0,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_BENDER           => 0xE0,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_SYSTEM_EXCLUSIVE => 0xF0,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_MTC_QUARTER      => 0xF1,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_SONG_POS         => 0xF2,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_SONG_SELECT      => 0xF3,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_TUNE_REQUEST     => 0xF6,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_CLOCK            => 0xF8,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_START            => 0xFA,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_CONTINUE         => 0xFB,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_STOP             => 0xFC,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_ACTIVE_SENSE     => 0xFE,
			LV2_Midi_Message_Type::LV2_MIDI_MSG_RESET            => 0xFF 
		}
    }
}
	
#[allow(non_camel_case_types)]	
pub enum LV2_Midi_Controller {
	LV2_MIDI_CTL_MSB_BANK             = 0x00,  
	LV2_MIDI_CTL_MSB_MODWHEEL         = 0x01,  
	LV2_MIDI_CTL_MSB_BREATH           = 0x02,  
	LV2_MIDI_CTL_MSB_FOOT             = 0x04,  
	LV2_MIDI_CTL_MSB_PORTAMENTO_TIME  = 0x05,  
	LV2_MIDI_CTL_MSB_DATA_ENTRY       = 0x06,  
	LV2_MIDI_CTL_MSB_MAIN_VOLUME      = 0x07,  
	LV2_MIDI_CTL_MSB_BALANCE          = 0x08,  
	LV2_MIDI_CTL_MSB_PAN              = 0x0A,  
	LV2_MIDI_CTL_MSB_EXPRESSION       = 0x0B,  
	LV2_MIDI_CTL_MSB_EFFECT1          = 0x0C,  
	LV2_MIDI_CTL_MSB_EFFECT2          = 0x0D,  
	LV2_MIDI_CTL_MSB_GENERAL_PURPOSE1 = 0x10,  
	LV2_MIDI_CTL_MSB_GENERAL_PURPOSE2 = 0x11,  
	LV2_MIDI_CTL_MSB_GENERAL_PURPOSE3 = 0x12,  
	LV2_MIDI_CTL_MSB_GENERAL_PURPOSE4 = 0x13,  
	LV2_MIDI_CTL_LSB_BANK             = 0x20,  
	LV2_MIDI_CTL_LSB_MODWHEEL         = 0x21,  
	LV2_MIDI_CTL_LSB_BREATH           = 0x22,  
	LV2_MIDI_CTL_LSB_FOOT             = 0x24,  
	LV2_MIDI_CTL_LSB_PORTAMENTO_TIME  = 0x25,  
	LV2_MIDI_CTL_LSB_DATA_ENTRY       = 0x26,  
	LV2_MIDI_CTL_LSB_MAIN_VOLUME      = 0x27,  
	LV2_MIDI_CTL_LSB_BALANCE          = 0x28,  
	LV2_MIDI_CTL_LSB_PAN              = 0x2A,  
	LV2_MIDI_CTL_LSB_EXPRESSION       = 0x2B,  
	LV2_MIDI_CTL_LSB_EFFECT1          = 0x2C,  
	LV2_MIDI_CTL_LSB_EFFECT2          = 0x2D,  
	LV2_MIDI_CTL_LSB_GENERAL_PURPOSE1 = 0x30,  
	LV2_MIDI_CTL_LSB_GENERAL_PURPOSE2 = 0x31,  
	LV2_MIDI_CTL_LSB_GENERAL_PURPOSE3 = 0x32,  
	LV2_MIDI_CTL_LSB_GENERAL_PURPOSE4 = 0x33,  
	LV2_MIDI_CTL_SUSTAIN              = 0x40,  
	LV2_MIDI_CTL_PORTAMENTO           = 0x41,  
	LV2_MIDI_CTL_SOSTENUTO            = 0x42,  
	LV2_MIDI_CTL_SOFT_PEDAL           = 0x43,  
	LV2_MIDI_CTL_LEGATO_FOOTSWITCH    = 0x44,  
	LV2_MIDI_CTL_HOLD2                = 0x45,  
	LV2_MIDI_CTL_SC1_SOUND_VARIATION  = 0x46,  
	LV2_MIDI_CTL_SC2_TIMBRE           = 0x47,  
	LV2_MIDI_CTL_SC3_RELEASE_TIME     = 0x48,  
	LV2_MIDI_CTL_SC4_ATTACK_TIME      = 0x49,  
	LV2_MIDI_CTL_SC5_BRIGHTNESS       = 0x4A,  
	LV2_MIDI_CTL_SC6                  = 0x4B,  
	LV2_MIDI_CTL_SC7                  = 0x4C,  
	LV2_MIDI_CTL_SC8                  = 0x4D,  
	LV2_MIDI_CTL_SC9                  = 0x4E,  
	LV2_MIDI_CTL_SC10                 = 0x4F,  
	LV2_MIDI_CTL_GENERAL_PURPOSE5     = 0x50,  
	LV2_MIDI_CTL_GENERAL_PURPOSE6     = 0x51,  
	LV2_MIDI_CTL_GENERAL_PURPOSE7     = 0x52,  
	LV2_MIDI_CTL_GENERAL_PURPOSE8     = 0x53,  
	LV2_MIDI_CTL_PORTAMENTO_CONTROL   = 0x54,  
	LV2_MIDI_CTL_E1_REVERB_DEPTH      = 0x5B,  
	LV2_MIDI_CTL_E2_TREMOLO_DEPTH     = 0x5C,  
	LV2_MIDI_CTL_E3_CHORUS_DEPTH      = 0x5D,  
	LV2_MIDI_CTL_E4_DETUNE_DEPTH      = 0x5E,  
	LV2_MIDI_CTL_E5_PHASER_DEPTH      = 0x5F,  
	LV2_MIDI_CTL_DATA_INCREMENT       = 0x60,  
	LV2_MIDI_CTL_DATA_DECREMENT       = 0x61,  
	LV2_MIDI_CTL_NRPN_LSB             = 0x62,  
	LV2_MIDI_CTL_NRPN_MSB             = 0x63,  
	LV2_MIDI_CTL_RPN_LSB              = 0x64,  
	LV2_MIDI_CTL_RPN_MSB              = 0x65,  
	LV2_MIDI_CTL_ALL_SOUNDS_OFF       = 0x78,  
	LV2_MIDI_CTL_RESET_CONTROLLERS    = 0x79,  
	LV2_MIDI_CTL_LOCAL_CONTROL_SWITCH = 0x7A,  
	LV2_MIDI_CTL_ALL_NOTES_OFF        = 0x7B,  
	LV2_MIDI_CTL_OMNI_OFF             = 0x7C,  
	LV2_MIDI_CTL_OMNI_ON              = 0x7D,  
	LV2_MIDI_CTL_MONO1                = 0x7E,  
	LV2_MIDI_CTL_MONO2                = 0x7F   
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
		_ => true
	}
}

/**
   Return the type of a MIDI message.
   @param msg Pointer to the start (status byte) of a MIDI message.
*/
pub fn lv2_midi_message_type(msg: &[u8]) -> LV2_Midi_Message_Type {
	if lv2_midi_is_voice_message(msg) {
		LV2_Midi_Message_Type::from_u8(msg[0] & 0xF0)
	} else if lv2_midi_is_system_message(msg) {
		LV2_Midi_Message_Type::from_u8(msg[0])
	} else {
		LV2_Midi_Message_Type::LV2_MIDI_MSG_INVALID
	}
}