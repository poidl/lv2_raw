extern crate libc;
extern crate lv2_raw;

use libc::*;
use std::mem;
use lv2_raw::*;

#[test]
fn it_works() {

    // how much memory must be allocated
    // sequence of two events
    // 1* size of LV2AtomSequence (includes header and body)
    // 2 * (size of event1)
    // 2 * 4 for pad
    // 2 * 4 for u32 data
    const N: usize = 64;
    let s_seq = mem::size_of::<LV2AtomSequence>() as isize;
    let s_ev = mem::size_of::<LV2AtomEvent>() as isize;
    let n = s_seq + 2*s_ev + 2*4 + 2*4;
    if n != N as isize {
        panic!("Need to adjust buffer size. Size is {}. Buffer is {}.",
               n,
               N)
    }

    let s_atom = mem::size_of::<LV2Atom>() as isize;
    println!("**************size ATOM: {}", s_atom);
    println!("**************size EVENT: {}", s_ev);
    let atom = LV2Atom {
        // Size in bytes, not including type and size.
        size: N as u32 - s_atom as u32,
        // Type of this atom (mapped URI).
        mytype: 0,
    };

    let seqbody = LV2AtomSequenceBody {
        // URID of unit of event time stamps.
        unit: 0,
        // Currently unused.
        pad: 0, // Contents (a series of events) follow here.
    };

    let sequence = LV2AtomSequence {
        // Atom header.
        atom: atom,
        // Body.
        body: seqbody,
    };

    /// ////////////////////////////////////
    let atom_ev1 = LV2Atom {
        size: 8,
        mytype: 7,
    };
    let atom_ev2 = LV2Atom {
        size: 8,
        mytype: 8,
    };
    let event1 = LV2AtomEvent {
        time_in_frames: 33333i64,
        body: atom_ev1,
    };
    let event2 = LV2AtomEvent {
        time_in_frames: 44444i64,
        body: atom_ev2,
    };
    // event pad  size is 64 bits. use 32 here and pad with an u16
    // then we have 16(that's s_ev)+16+32 = 64
    let atomdata1 = 11u32;
    let atomdata2 = 22u32;
    let pad = 0u32;


    let mut buf = [1u8; N];

    unsafe {
        let p1 = &mut buf[0] as *mut u8 as *mut libc::c_void;
        let p2 = &sequence as *const LV2AtomSequence as *const libc::c_void;
        libc::memcpy(p1, p2, s_seq as usize);


        let p2 = &event1 as *const LV2AtomEvent as *const libc::c_void;
        libc::memcpy(p1.offset(s_seq), p2, s_ev as usize);
        let p2 = &pad as *const u32 as *const libc::c_void;
        libc::memcpy(p1.offset(s_seq + s_ev), p2, 4);
        let p2 = &atomdata1 as *const u32 as *const libc::c_void;
        libc::memcpy(p1.offset(s_seq + s_ev + 4), p2, 4);

        let p2 = &event2 as *const LV2AtomEvent as *const libc::c_void;
        libc::memcpy(p1.offset(s_seq + s_ev + 4 + 4), p2, s_ev as usize);
        let p2 = &pad as *const u32 as *const libc::c_void;
        libc::memcpy(p1.offset(s_seq + 2*s_ev + 4 + 4), p2, 4);
        let p2 = &atomdata2 as *const u32 as *const libc::c_void;
        libc::memcpy(p1.offset(s_seq + 2*s_ev + 2*4 + 4), p2, 4);


        let seq = &buf[0] as *const u8 as *const LV2AtomSequence;
        for ev in &*seq {
            println!{"*************TIME: {}", ev.time_in_frames}
            println!{"*************ATOM.MYTYPE: {}", ev.body.mytype}
            let atomptr = &ev.body as *const LV2Atom as *const u8;
            let dataptr = atomptr.offset(s_atom+4);
            let data = *(dataptr as *const u32);
            println!{"************ data: {}", data};
        }
    }
    assert_eq!(4, 4);
}
