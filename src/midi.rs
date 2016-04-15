pub type MidiMessage<'a> = &'a u8;

pub trait MidiTranslate {
    fn noteon(&self) -> bool;
    fn noteoff(&self) -> bool;
    fn cc(&self) -> bool;

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
