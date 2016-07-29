extern crate libc;
// use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
// use plugin;
use midi;
use midi::*;
use yassy::plugin;
use std::str;
use jack::*;

#[repr(C)]
pub struct jack_plugin<'a> {
    name: &'a CString,
    pub client: *mut JackClientT,
    pub in_port: Port,
    pub output: Port,
    pub plugin: plugin::SynthPlugin, /*     pub output: *mut f32,
                                      * ports: &'a [Port] */
    param_values: [f32; 1],
}

pub struct Port {
    pub handle: *mut JackPortT,
    pub data: *mut f32,
}

impl<'a> jack_plugin<'a> {
    pub fn new(hostname: &CString) -> jack_plugin {
        let mut h = jack_plugin {
            name: hostname,
            client: ptr::null_mut(),
            in_port: Port {
                handle: ptr::null_mut(),
                data: ptr::null_mut(),
            },
            output: Port {
                handle: ptr::null_mut(),
                data: ptr::null_mut(),
            },
            plugin: plugin::SynthPlugin::new(),
            param_values: [0.5f32],
        };

        let jo = JACK_NULL_OPTION;
        let js = JACK_NULL_STATUS;
        // let cstr = CString::new(h.name).unwrap();
        let nameptr = h.name.as_ptr();
        unsafe {
            h.client = jack_client_open(nameptr, jo, &js);
        }
        if h.client == 0 as *mut JackClientT {
            println!("jack server not running?");
            // TODO return error
        };
        h
    }
    pub fn initialize(&mut self) {
        // Cannot initialize parameter pointers in new(), since they
        // point to memory local to the stack of new()!!
        self.plugin.params[plugin::ParamName::Gain as usize] = &mut self.param_values[0];
    }
    pub fn connect(&mut self) {
        unsafe {
            let portname = CString::new("midi_in").unwrap();
            let porttype = CString::new("8 bit raw midi").unwrap();
            self.in_port.handle = jack_port_register(self.client,
                                                     portname.as_ptr(),
                                                     porttype.as_ptr(),
                                                     JACK_PORT_IS_INPUT,
                                                     0u64);

            let portname = CString::new("audio_out").unwrap();
            let porttype = CString::new("32 bit float mono audio").unwrap();
            self.output.handle = jack_port_register(self.client,
                                                    portname.as_ptr(),
                                                    porttype.as_ptr(),
                                                    JACK_PORT_IS_OUTPUT,
                                                    0u64);
        }
    }
    pub fn midievent(&mut self, msg: &u8) {

        let mm = msg as midi::MidiMessage;
        if mm.noteon() {
            self.plugin.noteon(mm.f0(), mm.vel())
        } else if mm.noteoff() {
            self.plugin.noteoff();
        } else if mm.cc() {
            let x = mm.cc_type();
            unsafe {
                match x {
                    midi::CcKind::ChannelVolume => {
                        *(self.plugin.params[plugin::ParamName::Gain as usize]) = mm.cc_value()
                    }
                    _ => println!("Don't understand cc midi message", ),
                }
            }
            println!("ccnr: {}", mm.ccnr());
            println!("ccval: {}", mm.ccval());
        } else {
            println!("Don't understand midi message", );
        }
    }
    pub fn set_fs(&mut self) {
        unsafe {
            let fs = jack_get_sample_rate(self.client);
            self.plugin.set_fs(fs as f64);
        }
    }
    pub fn get_amp(&mut self) -> f32 {
        self.plugin.get_amp()
    }
}
