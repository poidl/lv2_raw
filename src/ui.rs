// Copyright 2017 Stefan Riha

// Documentation copied from http://lv2plug.in/ns/extensions/ui/ui.h

// Copyright text of the original C file:

// LV2 UI Extension
// Copyright 2009-2016 David Robillard <d@drobilla.net>
// Copyright 2006-2011 Lars Luthman <lars.luthman@gmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THIS SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

//! Documentation of the corresponding C header files: http://lv2plug.in/ns/extensions/ui/ui.html.

use libc;
use core::*;

/**
   A pointer to UI instance internals.

   The host may compare this to NULL, but otherwise MUST NOT interpret it.
*/

pub type LV2UIHandle = *mut libc::c_void;

/**
   A pointer to some widget or other type of UI handle.

   The actual type is defined by the type of the UI.
*/
pub type LV2UIWidget = *mut libc::c_void;

/**
   A pointer to a controller provided by the host.

   The UI may compare this to NULL, but otherwise MUST NOT interpret it.
*/
pub type LV2UIControllerRaw = *const libc::c_void;

/**
   A host-provided function that sends data to a plugin's input ports.

   @param controller The opaque controller pointer passed to
   LV2UI_Descriptor::instantiate().

   @param port_index Index of the port to update.

   @param buffer Buffer containing `buffer_size` bytes of data.

   @param buffer_size Size of `buffer` in bytes.

   @param port_protocol Either 0 or the URID for a ui:PortProtocol.  If 0, the
   protocol is implicitly ui:floatProtocol, the port MUST be an lv2:ControlPort
   input, `buffer` MUST point to a single float value, and `buffer_size` MUST
   be sizeof(float).  The UI SHOULD NOT use a protocol not supported by the
   host, but the host MUST gracefully ignore any protocol it does not
   understand.
*/
pub type LV2UIWriteFunctionRaw = Option<extern "C" fn(controller: LV2UIControllerRaw,
                                                      port_index: libc::c_uint,
                                                      buffer_size: libc::c_uint,
                                                      port_protocol: libc::c_uint,
                                                      buffer: *const libc::c_void)>;

/**
   A plugin UI.

   A pointer to an object of this type is returned by the lv2ui_descriptor()
   function.
*/
#[repr(C)]
pub struct LV2UIDescriptorRaw {
    /**
	   The URI for this UI (not for the plugin it controls).
	*/
    pub uri: *const libc::c_char,

    /**
	   Create a new UI and return a handle to it.  This function works
	   similarly to LV2_Descriptor::instantiate().

	   @param descriptor The descriptor for the UI to instantiate.

	   @param plugin_uri The URI of the plugin that this UI will control.

	   @param bundle_path The path to the bundle containing this UI, including
	   the trailing directory separator.

	   @param write_function A function that the UI can use to send data to the
	   plugin's input ports.

	   @param controller A handle for the UI instance to be passed as the
	   first parameter of UI methods.

	   @param widget (output) widget pointer.  The UI points this at its main
	   widget, which has the type defined by the UI type in the data file.

	   @param features An array of LV2_Feature pointers.  The host must pass
	   all feature URIs that it and the UI supports and any additional data, as
	   in LV2_Descriptor::instantiate().  Note that UI features and plugin
	   features are not necessarily the same.

	*/
    pub instantiate_raw: extern "C" fn(descriptor: *const LV2UIDescriptorRaw,
                                           plugin_uri: *const libc::c_char,
                                           bundle_path: *const libc::c_char,
                                           write_function: LV2UIWriteFunctionRaw,
                                           controller: LV2UIControllerRaw,
                                           widget: *mut LV2UIWidget,
                                           features: *const (*const LV2Feature))
                                           -> LV2UIHandle,

    /**
	   Destroy the UI.  The host must not try to access the widget after
	   calling this function.
	*/
    pub cleanup: extern "C" fn(LV2UIHandle),

    /**
	   Tell the UI that something interesting has happened at a plugin port.

	   What is "interesting" and how it is written to `buffer` is defined by
	   `format`, which has the same meaning as in LV2UI_Write_Function().
	   Format 0 is a special case for lv2:ControlPort, where this function
	   should be called when the port value changes (but not necessarily for
	   every change), `buffer_size` must be sizeof(float), and `buffer`
	   points to a single IEEE-754 float.

	   By default, the host should only call this function for lv2:ControlPort
	   inputs.  However, the UI can request updates for other ports statically
	   with ui:portNotification or dynamicaly with ui:portSubscribe.

	   The UI MUST NOT retain any reference to `buffer` after this function
	   returns, it is only valid for the duration of the call.

	   This member may be NULL if the UI is not interested in any port events.
	*/
    pub port_event: extern "C" fn(ui: LV2UIHandle,
                                      port_index: libc::c_uint,
                                      buffer_size: libc::c_uint,
                                      format: libc::c_uint,
                                      buffer: *const libc::c_void),

    /**
	   Return a data structure associated with an extension URI, typically an
	   interface struct with additional function pointers

	   This member may be set to NULL if the UI is not interested in supporting
	   any extensions. This is similar to LV2_Descriptor::extension_data().

	*/
    pub extension_data: Option<extern "C" fn(*const libc::c_char) -> *const libc::c_void>,
}


/**
   UI Idle Interface (LV2_UI__idleInterface)

   UIs can provide this interface to have an idle() callback called by the host
   rapidly to update the UI.
*/
#[repr(C)]
pub struct LV2UIIdleInterface {
    /**
	   Run a single iteration of the UI's idle loop.

	   This will be called rapidly in the UI thread at a rate appropriate
	   for a toolkit main loop.  There are no precise timing guarantees, but
	   the host should attempt to call idle() at a high enough rate for smooth
	   animation, at least 30Hz.

	   @return non-zero if the UI has been closed, in which case the host
	   should stop calling idle(), and can either completely destroy the UI, or
	   re-show it and resume calling idle().
	*/
    pub idle: extern "C" fn(ui: LV2UIHandle) -> libc::c_int,
}

/**
   UI Show Interface (LV2_UI__showInterface)

   UIs can provide this interface to show and hide a window, which allows them
   to function in hosts unable to embed their widget.  This allows any UI to
   provide a fallback for embedding that works in any host.

   If used:
   - The host MUST use LV2UI_Idle_Interface to drive the UI.
   - The UI MUST return non-zero from LV2UI_Idle_Interface::idle() when it has been closed.
   - If idle() returns non-zero, the host MUST call hide() and stop calling
     idle().  It MAY later call show() then resume calling idle().
*/
#[repr(C)]
pub struct LV2UIShowInterface {
    /**
	   Show a window for this UI.

	   The window title MAY have been passed by the host to
	   LV2UI_Descriptor::instantiate() as an LV2_Options_Option with key
	   LV2_UI__windowTitle.

	   @return 0 on success, or anything else to stop being called.
	*/
    pub show: extern "C" fn(ui: LV2UIHandle) -> libc::c_int,

    /**
	   Hide the window for this UI.

	   @return 0 on success, or anything else to stop being called.
	*/
    pub hide: extern "C" fn(ui: LV2UIHandle) -> libc::c_int,
}



// RUST_TODO: The following are deprecated, should not declare this here. Ardour and Qtractor do not implement ui:showInterface
// http://lists.lv2plug.in/pipermail/devel-lv2plug.in/2016-May/001649.html
// http://kxstudio.linuxaudio.org/ns/lv2ext/lv2_external_ui.h

/**
 * When LV2_EXTERNAL_UI__Widget UI is instantiated, the returned
 * LV2UI_Widget handle must be cast to pointer to LV2_External_UI_Widget.
 * UI is created in invisible state.
 */
#[repr(C)]
pub struct LV2UIExternalUIWidget {
    /**
   * Host calls this function regulary. UI library implementing the
   * callback may do IPC or redraw the UI.
   *
   * @param _this_ the UI context
   */
    pub run: Option<extern "C" fn(ui: *const LV2UIExternalUIWidget)>,

    /**
   * Host calls this function to make the plugin UI visible.
   *
   * @param _this_ the UI context
   */
    pub show: Option<extern "C" fn(ui: *const LV2UIExternalUIWidget)>,

    /**
   * Host calls this function to make the plugin UI invisible again.
   *
   * @param _this_ the UI context
   */
    pub hide: Option<extern "C" fn(ui: *const LV2UIExternalUIWidget)>,
}

/**
 * On UI instantiation, host must supply LV2_EXTERNAL_UI__Host feature.
 * LV2_Feature::data must be pointer to LV2_External_UI_Host.
 */
#[repr(C)]
pub struct LV2UIExternalUIHost {
    /**
   * Callback that plugin UI will call when UI (GUI window) is closed by user.
   * This callback will be called during execution of LV2_External_UI_Widget::run()
   * (i.e. not from background thread).
   *
   * After this callback is called, UI is defunct. Host must call LV2UI_Descriptor::cleanup().
   * If host wants to make the UI visible again, the UI must be reinstantiated.
   *
   * @note When using the depreated URI LV2_EXTERNAL_UI_DEPRECATED_URI,
   *       some hosts will not call LV2UI_Descriptor::cleanup() as they should,
   *       and may call show() again without re-initialization.
   *
   * @param controller Host context associated with plugin UI, as
   *                   supplied to LV2UI_Descriptor::instantiate().
   */
    pub ui_closed: extern "C" fn(host: LV2UIControllerRaw) -> libc::c_void,

    /**
   * Optional (may be NULL) "user friendly" identifier which the UI
   * may display to allow a user to easily associate this particular
   * UI instance with the correct plugin instance as it is represented
   * by the host (e.g. "track 1" or "channel 4").
   *
   * If supplied by host, the string will be referenced only during
   * LV2UI_Descriptor::instantiate()
   */
    pub plugin_human_id: *const libc::c_char,
}
