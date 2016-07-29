pub type MidiMessage<'a> = &'a u8;

pub enum CcKind {
    ChannelVolume,
    Unknown
}

pub trait MidiTranslate {
    fn noteon(&self) -> bool;
    fn noteoff(&self) -> bool;
    fn cc(&self) -> bool;
    fn cc_type(&self) -> CcKind;
    fn cc_value(&self) -> f32;

    fn f0(&self) -> f32;
    fn vel(&self) -> f32;

    fn ccnr(&self) -> u8;
    fn ccval(&self) -> u8;
}

impl<'a> MidiTranslate for MidiMessage<'a> {
    fn noteon(&self) -> bool {
        *self & 0xf0 == 0x90
    }
    fn noteoff(&self) -> bool {
        *self & 0xf0 == 0x80
    }
    fn cc(&self) -> bool {
        *self & 0xf0 == 0xb0
    }
    fn cc_type(&self) -> CcKind {
        let msg = *self as *const u8;
        unsafe{
            let x = *msg.offset(1);
            match x {
                0x07 => return CcKind::ChannelVolume,
                _    => return CcKind::Unknown
            }
        }
    }
    fn cc_value(&self) -> f32 {
        let msg = *self as *const u8;
        unsafe{
            let x = *msg.offset(2);
            (x as f32)/127f32
        }
    }
    fn f0(&self) -> f32 {
        let msg = *self as *const u8;
        unsafe{
            let i = *msg.offset(1);
            let f0 = (2.0f32.powf((((i as i8)-57) as f32)/12.0))*220.0;
            return f0
        }
    }
    fn vel(&self) -> f32 {
        let msg = *self as *const u8;
        unsafe{
            let i = *msg.offset(2);
            return i as f32 / 127 as f32
        }
    }
    fn ccnr(&self) -> u8 {
        let msg = *self as *const u8;
        unsafe{
            *msg.offset(1)
        }
    }
    fn ccval(&self) -> u8 {
        let msg = *self as *const u8;
        unsafe{
            *msg.offset(2)
        }
    }
}
