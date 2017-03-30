
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
	
