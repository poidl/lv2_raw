use libc;
use core::*;

pub type LV2UIHandle = *mut libc::c_void;
pub type LV2UIWidget = *mut libc::c_void;
pub type LV2UIController = *const libc::c_void;
pub type LV2UIWriteFunction = extern "C" fn(controller: LV2UIController,
                                            port_index: libc::c_uint,
                                            buffer_size: libc::c_uint,
                                            port_protocol: libc::c_uint,
                                            buffer: *const libc::c_void);

#[repr(C)]
pub struct LV2UIIdleInterface {
    pub idle: extern "C" fn(ui: LV2UIHandle) -> libc::c_int,
}

#[repr(C)]
pub struct LV2UIShowInterface {
    pub show: extern "C" fn(ui: LV2UIHandle) -> libc::c_int,
    pub hide: extern "C" fn(ui: LV2UIHandle) -> libc::c_int,
}


// TODO: This is deprecated, should not declare this here. Ardour and Qtractor do not implement
// ui:showInterface
// http://lists.lv2plug.in/pipermail/devel-lv2plug.in/2016-May/001649.html
// http://kxstudio.linuxaudio.org/ns/lv2ext/lv2_external_ui.h
#[repr(C)]
pub struct LV2UIExternalUIWidget {
    // Why "Option"? Nullable function pointers. See
    // https://doc.rust-lang.org/book/ffi.html
    // https://mail.mozilla.org/pipermail/rust-dev/2014-September/011200.html
    pub run: Option<extern "C" fn(ui: LV2UIExternalUIWidget)>,
    pub show: Option<extern "C" fn(ui: LV2UIExternalUIWidget)>,
    pub hide: Option<extern "C" fn(ui: LV2UIExternalUIWidget)>,
}

#[repr(C)]
pub struct LV2UIExternalUIHost {
    pub ui_closed: extern "C" fn(host: LV2UIController) -> libc::c_void,
    pub plugin_human_id: *const libc::c_char,
}

#[repr(C)]
pub struct LV2UIDescriptor {
    pub uri: *const libc::c_char,
    pub instantiate: extern "C" fn(descriptor: *const LV2UIDescriptor,
                                   plugin_uri: *const libc::c_char,
                                   bundle_path: *const libc::c_char,
                                   write_function: LV2UIWriteFunction,
                                   controller: LV2UIController,
                                   widget: *mut LV2UIWidget,
                                   features: *const (*const LV2Feature))
                                   -> LV2UIHandle,
    pub cleanup: extern "C" fn(LV2UIHandle),
    pub port_event: extern "C" fn(ui: LV2UIHandle,
                                  port_index: libc::c_uint,
                                  buffer_size: libc::c_uint,
                                  format: libc::c_uint,
                                  buffer: *const libc::c_void),
    // Why "Option"? Nullable function pointers. See
    // https://doc.rust-lang.org/book/ffi.html
    // https://mail.mozilla.org/pipermail/rust-dev/2014-September/011200.html
    pub extension_data: Option<extern "C" fn(*const libc::c_char) -> *const libc::c_void>,
}
