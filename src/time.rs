// Copyright 2017 Michael Oswald

// Documentation copied from http://lv2plug.in/ns/ext/time/time.h

// Copyright text of the original C file:

// Copyright 2011-2016 David Robillard <http://drobilla.net>

// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.

// THIS SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

/**
   Properties for describing time, see <http://lv2plug.in/ns/ext/time> for
   details.

   Note the time extension is purely data, this header merely defines URIs for
   convenience.
*/

pub static LV2_TIME_URI : &'static [u8] =  b"http://lv2plug.in/ns/ext/time\0";
pub static LV2_TIME_PREFIX : &'static [u8] = b"http://lv2plug.in/ns/ext/time#\0";

pub static LV2_TIME__TIME            : &'static [u8] = b"http://lv2plug.in/ns/ext/time#Time\0";
pub static LV2_TIME__POSITION        : &'static [u8] = b"http://lv2plug.in/ns/ext/time#Position\0";
pub static LV2_TIME__RATE            : &'static [u8] = b"http://lv2plug.in/ns/ext/time#Rate\0";
pub static LV2_TIME___POSITION        : &'static [u8] = b"http://lv2plug.in/ns/ext/time#position\0";
pub static LV2_TIME__BARBEAT         : &'static [u8] = b"http://lv2plug.in/ns/ext/time#barBeat\0";
pub static LV2_TIME__BAR             : &'static [u8] = b"http://lv2plug.in/ns/ext/time#bar\0";
pub static LV2_TIME__BEAT            : &'static [u8] = b"http://lv2plug.in/ns/ext/time#beat\0";
pub static LV2_TIME__BEATUNIT        : &'static [u8] = b"http://lv2plug.in/ns/ext/time#beatUnit\0";
pub static LV2_TIME__BEATSPERBAR     : &'static [u8] = b"http://lv2plug.in/ns/ext/time#beatsPerBar\0";
pub static LV2_TIME__BEATSPERMINUTE  : &'static [u8] = b"http://lv2plug.in/ns/ext/time#beatsPerMinute\0";
pub static LV2_TIME__FRAME           : &'static [u8] = b"http://lv2plug.in/ns/ext/time#frame\0";
pub static LV2_TIME__FRAMESPERSECOND : &'static [u8] = b"http://lv2plug.in/ns/ext/time#framesPerSecond\0";
pub static LV2_TIME__SPEED           : &'static [u8] = b"http://lv2plug.in/ns/ext/time#speed\0";
