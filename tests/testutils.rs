extern crate libc;
extern crate lv2_raw;

// use libc::*;
use std::mem;
use lv2_raw::*;

// Need to define constant for buffer allocation, which must be identical
// to n
const N: usize = 64;

#[test]
fn it_works() {

    // Construct a sequence of two events by hand:

    // How much memory must be allocated?
    // 1 * (size of LV2AtomSequence)
    // 2 * (size of event1)
    // 2 * 8 for one u64 data per atom

    let s_seq = mem::size_of::<LV2AtomSequence>() as isize;
    let s_ev = mem::size_of::<LV2AtomEvent>() as isize;
    let s_atom = 8 as isize;
    let n = s_seq + 2*s_ev + 2*s_atom;
    if n != N as isize {
        panic!("Need to adjust buffer size. Size is {}. Buffer is {}.",
               n,
               N)
    }

    let s_atom_header = mem::size_of::<LV2Atom>() as isize;

    let atom = LV2Atom {
        // Size in bytes, not including type and size.
        size: N as u32 - s_atom_header as u32,
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

    ////////////////////////////////////////
    let atom_ev1 = LV2Atom {
        size: s_atom as u32,
        mytype: 7, // some random type
    };
    let atom_ev2 = LV2Atom {
        size: s_atom as u32,
        mytype: 8, 
    };
    let event1 = LV2AtomEvent {
        time_in_frames: 33333i64, // some random timestamp
        body: atom_ev1,
    };
    let event2 = LV2AtomEvent {
        time_in_frames: 44444i64,
        body: atom_ev2,
    };
    // event pad size is 64 bits, using u64 no is padding necessary
    let atomdata1 = 11u64; // some random data
    let atomdata2 = 22u64;

    let buf = [1u8; N];

    unsafe {

        let mut state = State{buf: buf, current: 0};

        let p = &sequence as *const LV2AtomSequence as *const libc::c_void;
        state.append(p, s_seq);

        // Event 1 
        let p = &event1 as *const LV2AtomEvent as *const libc::c_void;
        state.append(p, s_ev);
        let p = &atomdata1 as *const u64 as *const libc::c_void;
        state.append(p, s_atom);

        // Event 2
        let p = &event2 as *const LV2AtomEvent as *const libc::c_void;
        state.append(p, s_ev);
        let p = &atomdata2 as *const u64 as *const libc::c_void;
        state.append(p, s_atom);

        // let seq = &buf[0] as *const u8 as *const LV2AtomSequence;
        let seq = &state.buf[0] as *const u8 as *const LV2AtomSequence;
        for ev in &*seq {
            println!{"*************TIME: {}", ev.time_in_frames}
            println!{"*************ATOM.MYTYPE: {}", ev.body.mytype}
            let atomptr = &ev.body as *const LV2Atom as *const u8;
            let dataptr = atomptr.offset(s_atom_header);
            let data = *(dataptr as *const u64);
            println!{"************ data: {}", data};
        }
    }
    assert_eq!(4, 4);
}

struct State {
    buf: [u8; N],
    current: isize
}

impl State {
    fn append(&mut self, p: *const libc::c_void, size: isize) {
        let p1 = &mut self.buf[0] as *mut u8 as *mut libc::c_void;
        unsafe {
            libc::memcpy(p1.offset(self.current), p, size as usize);
        }
        self.current = self.current + size;
    }
}