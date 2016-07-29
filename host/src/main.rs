#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate websocket;
extern crate yassy;
extern crate midi;

mod jack;
mod jack_plugin;
use jack::*;

use std::time::Duration;
use std::ffi::CString;
// use std::thread;
// use websocket::{Server, Message, Sender, Receiver};
// use websocket::message::Type;
// use websocket::header::WebSocketProtocol;

// use std::ffi::CString;
// use std::ffi::CStr;
// use std::sync::{Arc, Mutex};
// use std::sync::mpsc;
// use std::ptr;
// use jack_plugin;
// use yassy::Plugin;

extern "C" fn process(jack_nframes_t: u32, ptr: *mut libc::c_void) -> isize {
    unsafe {
        let plugin = ptr as *mut Plugin;

        let inport = &(*plugin).in_port;
        let outport = &(*plugin).output;
        let buf = jack_port_get_buffer(inport.handle, jack_nframes_t);
        let out = jack_port_get_buffer(outport.handle, jack_nframes_t) as *mut f32;
        let event_count = jack_midi_get_event_count(buf);

        let mut event = JackMidiEvent {
            time: 0,
            size: 0,
            buffer: std::ptr::null_mut() as *mut libc::c_uchar,
        };

        let mut ievent = 0;
        jack_midi_event_get(&mut event, buf, ievent);
        for i in 0..jack_nframes_t {
            if (event.time == i) & (ievent < event_count) {
                (*plugin).midievent(&*event.buffer);
                ievent = ievent + 1;
                // Need to check if ievent < event_count before next call to
                // jack_midi_event_get()? Don't think so, but see
                // https://github.com/jackaudio/example-clients/blob/master/midisine.c
                jack_midi_event_get(&mut event, buf, ievent);
            }
            let amp = (*plugin).get_amp();
            *out.offset(i as isize) = amp;
        }
    }
    0
}

pub type Plugin<'a> = jack_plugin::jack_plugin<'a>;

fn main() {
    // CString must stay alive after pointer is obtained
    // see http://stackoverflow.com/questions/38007154/jack-audio-client-name-longer-than-4-characters-breaks-client
    let name = CString::new("yassyhost").unwrap();
    let mut p = Plugin::new(&name);
    p.initialize();

    p.set_fs();
    p.connect();

    let cbpluginptr = (&p as *const Plugin) as *const libc::c_void;
    unsafe {
        jack_set_process_callback(p.client, process, cbpluginptr);
        jack_activate(p.client);
        let five = Duration::new(5, 0);
        loop {
            std::thread::sleep(five);
        }
        // jack_port_unregister(p.client, p.in_port.handle);
        // jack_port_unregister(p.client, p.output.handle);
        // jack_client_close(p.client);
    }
}
