# Exposes the raw LV2 interface (C language) to Rust

LV2 docs: http://lv2plug.in/

The documentation of this crate is copied from the original C files, whose
copyright holders include Steve Harris, Lars Luthman, Gabriel M. Beddingfield, David Robillard, Richard W.E. Furse, Paul Barton-Davis, Stefan Westerfeld, and possibly others.

## Note

The objective of this crate is to translate the C interface as closely as
possible to Rust, declaring e.g.

* type aliases for raw pointers
* `[repr(C)] struct`s
* `extern "C" fn` declarations passed to the host in the LV2Descriptor struct

The original (C language) LV2 package defines contains some "helper" functions, which are
defined in C-headers, i.e. there is no compiled library file which this crate 
could link to. These functions are also declared and defined here. Let us know if 
you think that's not right. See also [this question](http://stackoverflow.com/questions/40944524/how-does-one-design-a-plugin-interface-for-digital-audio-workstation-hosts-in-pu) on stackoverflow.

The [lv2 crate](https://crates.io/crates/lv2) is one attempt to provide a more
idiomatic Rust interface to LV2.

Roadmap:

- Figure out division line between low- and high-level crate
  - Preliminary goal: provide all functionality of LV2 while keeping it low-level
- Find out how much abstraction is useful/affordable in audio applications (i.e. real-time).

