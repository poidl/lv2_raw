use libc;
// adapted from libjack bindings for Rust
// Copyright (C) 2015 Nick Lanham


bitflags! {
    #[repr(C)]
    pub flags JackOptions: u32 {
        const JACK_NULL_OPTION   = 0x000000,
        // const JACK_NO_START_SERVER = 0x000001,
        // const JACK_USE_EXACT_NAME  = 0x000010,
        // const JACK_SERVER_NAME    = 0x000100,
        // const JACK_LOAD_NAME      = 0x001000,
        // const JACK_LOAD_INIT      = 0x010000,
        // const JACK_SESSION_ID     = 0x100000
    }
}

bitflags!(
    #[repr(C)]
    pub flags JackStatus: u32 {
const JACK_NULL_STATUS = 0x00,
// const JackFailure = 0x01,
// const JackInvalidOption = 0x02,
// const JackNameNotUnique = 0x04,
// const JackServerStarted = 0x08,
// const JackServerFailed = 0x10,
// const JackServerError = 0x20,
// const JackNoSuchClient = 0x40,
// const JackLoadFailure = 0x80,
// const JackInitFailure = 0x100,
// const JackShmFailure = 0x200,
// const JackVersionError = 0x400,
// const JackBackendError = 0x800,
// const JackClientZombie = 0x1000
    }
);

bitflags!(
    #[repr(C)]
    pub flags JackPortFlags: u32 {
        const JACK_PORT_IS_INPUT = 0x1,
        const JACK_PORT_IS_OUTPUT = 0x2,
// const JackPortIsPhysical = 0x4,
// const JackPortCanMonitor = 0x8,
// const JackPortIsTerminal = 0x10,
    }
);


// #[repr(C)]
// pub struct JackClientT; // WARNING: unsupported representation for zero-variant enum
pub enum JackClientT {}
// #[repr(C)]
// pub struct JackPortT;
pub enum JackPortT {}
#[repr(C)]
pub struct JackMidiEvent {
    pub time: u32,
    pub size: libc::size_t,
    pub buffer: *mut libc::c_uchar,
}



pub type JackProcessCallback = extern "C" fn(jack_nframes_t: u32, *mut libc::c_void) -> isize;

#[link(name = "jack")]
extern "C" {
    // core
    pub fn jack_client_open(name: *const libc::c_char,
                            options: JackOptions,
                            status: &JackStatus)
                            -> *mut JackClientT;
    pub fn jack_set_process_callback(client: *mut JackClientT,
                                     callback: JackProcessCallback,
                                     arg: *const libc::c_void)
                                     -> libc::c_int;
    pub fn jack_port_register(client: *mut JackClientT,
                              port_name: *const libc::c_char,
                              port_type: *const libc::c_char,
                              flags: JackPortFlags,
                              buffer_size: libc::c_ulong)
                              -> *mut JackPortT;
    // pub fn jack_port_unregister(client: *mut JackClientT, port: *mut JackPortT) -> u32;

    pub fn jack_activate(client: *mut JackClientT) -> libc::c_int;
    pub fn jack_port_get_buffer(port: *mut JackPortT, nframes: u32) -> *mut libc::c_void;
    pub fn jack_midi_get_event_count(port_buffer: *mut ::libc::c_void) -> u32;
    pub fn jack_midi_event_get(event: *mut JackMidiEvent,
                               port_buffer: *mut ::libc::c_void,
                               event_index: u32)
                               -> libc::c_int;
    pub fn jack_get_sample_rate(client: *const JackClientT) -> u32;
// pub fn jack_client_close(client: *const JackClientT) -> u32;
// pub fn jack_client_name_size() -> u32;
}
