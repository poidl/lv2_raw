extern crate libc;
extern crate lv2_raw;

use lv2_raw::*;
use std::mem;

// Need to define constant for buffer allocation, which must be identical
// to n
const N: usize = 64;

const TYPE1: u32 = 7; // random type
const TYPE2: u32 = 8;
const TIME1: i64 = 33333; // random data
const TIME2: i64 = 44444;
// event pad size is 64 bits, using u64 no is padding necessary
const ATOMDATA1: u64 = 11; // random data
const ATOMDATA2: u64 = 22;

fn get_buf() -> State {
    // Construct a sequence of two events by hand:

    // How much memory must be allocated?
    // 1 * (size of LV2AtomSequence)
    // 2 * (size of event1)
    // 2 * 8 for one u64 data per atom

    let s_seq = mem::size_of::<LV2AtomSequence>() as isize;
    let s_ev = mem::size_of::<LV2AtomEvent>() as isize;
    let s_atom = 8 as isize;
    let n = s_seq + 2 * s_ev + 2 * s_atom;
    if n != N as isize {
        panic!(
            "Need to adjust buffer size. Size is {}. Buffer is {}.",
            n, N
        )
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
        mytype: TYPE1,
    };
    let atom_ev2 = LV2Atom {
        size: s_atom as u32,
        mytype: TYPE2,
    };
    let event1 = LV2AtomEvent {
        time_in_frames: TIME1,
        body: atom_ev1,
    };
    let event2 = LV2AtomEvent {
        time_in_frames: TIME2,
        body: atom_ev2,
    };

    let buf = [1u8; N];

    let mut state = State {
        buf: buf,
        current: 0,
    };

    let p = &sequence as *const LV2AtomSequence as *const libc::c_void;
    state.append(p, s_seq);

    // Event 1
    let p = &event1 as *const LV2AtomEvent as *const libc::c_void;
    state.append(p, s_ev);
    let p = &ATOMDATA1 as *const u64 as *const libc::c_void;
    state.append(p, s_atom);

    // Event 2
    let p = &event2 as *const LV2AtomEvent as *const libc::c_void;
    state.append(p, s_ev);
    let p = &ATOMDATA2 as *const u64 as *const libc::c_void;
    state.append(p, s_atom);

    state
}

#[test]
fn it_works() {
    let truth = [
        TIME1 as u64,
        TYPE1 as u64,
        ATOMDATA1,
        TIME2 as u64,
        TYPE2 as u64,
        ATOMDATA2,
    ];

    let s_atom_header = mem::size_of::<LV2Atom>() as isize;

    let mut cnt = 0;
    let state = get_buf();

    unsafe {
        // next line basically says
        //  "let seq = &state.buf[0] as &LV2AtomSequence;"
        // but that's not allowed by the compiler
        let seq = &*(&state.buf[0] as *const u8 as *const LV2AtomSequence);
        for ev in seq {
            println! {"*************TIME: {}", ev.time_in_frames}
            assert_eq!(ev.time_in_frames as u64, truth[cnt]);

            println! {"*************ATOM.MYTYPE: {}", ev.body.mytype}
            assert_eq!(ev.body.mytype as u64, truth[cnt + 1]);

            let atomptr = &ev.body as *const LV2Atom as *const u8;

            let dataptr = atomptr.offset(s_atom_header);
            let data = *(dataptr as *const u64);
            println! {"************ data: {}", data};
            assert_eq!(data as u64, truth[cnt + 2]);

            cnt = cnt + 3;
        }
        // did we really loop throuh *2* events?
        assert_eq!(cnt, 6)
    }
}

struct State {
    buf: [u8; N],
    current: isize,
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
