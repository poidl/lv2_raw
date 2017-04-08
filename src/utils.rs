

use atom::*;
use std::mem::{size_of};
use libc::{memcmp, memcpy, c_void};

/// Pad a size to 64 bits
pub fn lv2_atom_pad_size(size: u32) -> u32 {
    (size + 7) & (!7)
}

/** Return the total size of `atom`, including the header. */
pub fn lv2_atom_total_size(atom: &LV2Atom) -> u32 {
    size_of::<LV2Atom>() as u32 + atom.size
}

/** Return true iff `atom` is null. */
pub unsafe fn lv2_atom_is_null(atom: *const LV2Atom) -> bool {
    atom.is_null() || ((*atom).mytype == 0 && (*atom).size == 0)
}

/** Return true iff `a` is equal to `b`. */
pub unsafe fn lv2_atom_equals(a: *const LV2Atom, b: *const LV2Atom) -> bool {
    (a == b) || (((*a).mytype == (*b).mytype) &&
                 ((*a).size == (*b).size) &&
                 (memcmp(a.offset(1) as *const c_void, 
                         b.offset(1) as *const c_void, 
                         (*a).size as usize) == 0))
}


/** Get an iterator pointing to the first event in a Sequence body. */
pub unsafe fn lv2_atom_sequence_begin(body: *const LV2AtomSequenceBody) -> *mut LV2AtomEvent {
    body.offset(1) as *mut LV2AtomEvent
}

/** Get an iterator pointing to the end of a Sequence body. */
pub unsafe fn lv2_atom_sequence_end(body: *const LV2AtomSequenceBody,
                                    size: u32) -> *const LV2AtomEvent {

    (body as *const u8).offset(lv2_atom_pad_size(size) as isize) as *const LV2AtomEvent
}

/** Return true iff `i` has reached the end of `body`. */
pub unsafe fn lv2_atom_sequence_is_end(body: *const LV2AtomSequenceBody,
            size: u32, i: *const LV2AtomEvent) -> bool {

    let result = i as *const u8 >= (body as *const u8).offset(size as isize);
    result
}


/** Return an iterator to the element following `i`. */
pub unsafe fn lv2_atom_sequence_next(i: *const LV2AtomEvent) -> *mut LV2AtomEvent {
    let off = size_of::<LV2AtomEvent>() + lv2_atom_pad_size((*i).body.size) as usize;
    let ptr = (i as *const u8).offset(off as isize);

    println!("lv2_atom_sequence_next: off: {} ptr: {:?}", off, ptr);

    ptr as *mut LV2AtomEvent
}

/**
   Clear all events from `sequence`.

   This simply resets the size field, the other fields are left untouched.
*/
pub unsafe fn lv2_atom_sequence_clear(seq: *mut LV2AtomSequence) -> () {
    (*seq).atom.size = size_of::<LV2AtomSequenceBody>() as u32;
}


/**
   Append an event at the end of `sequence`.

   @param seq Sequence to append to.
   @param capacity Total capacity of the sequence atom
   (e.g. as set by the host for sequence output ports).
   @param event Event to write.

   @return A pointer to the newly written event in `seq`,
   or NULL on failure (insufficient space).
*/
pub unsafe fn lv2_atom_sequence_append_event(seq: *mut LV2AtomSequence,
    capacity: u32, event: *const LV2AtomEvent) -> *const LV2AtomEvent {

    let total_size = size_of::<LV2AtomEvent>() as u32 + (*event).body.size;

    if (capacity - (*seq).atom.size) < total_size {
        return 0 as *const LV2AtomEvent;
    }

    let e = lv2_atom_sequence_end(&(*seq).body, (*seq).atom.size);
    memcpy(e as *mut c_void, event as *const c_void, total_size as usize);

    (*seq).atom.size += lv2_atom_pad_size(total_size);

    e
}

/** Return a pointer to the first property in `body`. */
pub unsafe fn lv2_atom_object_begin(body: *const LV2AtomObjectBody) -> 
    *mut LV2AtomPropertyBody {
    
    body.offset(1) as *mut LV2AtomPropertyBody
}

/** Return true iff `i` has reached the end of `obj`. */
pub unsafe fn lv2_atom_object_is_end(body: *const LV2AtomObjectBody,
    size: u32, i: *const LV2AtomPropertyBody) -> bool {

    i as *const u8 >= (body as *const u8).offset(size as isize)
}

/** Return an iterator to the property following `i`. */
pub unsafe fn lv2_atom_object_next(i: *const LV2AtomPropertyBody) -> *mut LV2AtomPropertyBody {

    let value = (i as *const u8).offset((2 * size_of::<u32>()) as isize) as *const LV2Atom;

    let offset = lv2_atom_pad_size(size_of::<LV2AtomPropertyBody>() as u32 + (*value).size);
    (i as *mut u8).offset(offset as isize) as *mut LV2AtomPropertyBody
}

/** A single entry in an Object query. */
pub struct LV2AtomObjectQuery {
    /**< Key to query (input set by user) */
    pub key: u32,
    /**< Found value (output set by query function) */
    pub value: *mut *mut LV2Atom
}


pub unsafe fn lv2_atom_object_query(obj: *mut LV2AtomObject, 
    query: *mut LV2AtomObjectQuery) -> i32 {

    let ref mut object = *obj;

    let mut n_queries = 0;
    let mut matches = 0;

    let q = query;
    while (*q).key != 0 {
        n_queries += 1;
        q.offset(1);
    }

    {
        let f = | prop: *mut LV2AtomPropertyBody | -> bool {
                    let mut q = query;
                    while (*q).key != 0 {

                        if ((*q).key == (*prop).key) && (!(*q).value.is_null()) {
                            let ref mut val = (*prop).value;
                            *(*q).value = val;
                            
                            matches += 1;
                            if matches == n_queries {
                                return true;
                            }
                            break;
                        }
                        q.offset(1);
                    }
                    false
                };

        object.foreach(f);
    }

    return matches;
}


pub struct ObjectHelper {
    pub key: u32,
    pub atom: *mut *mut LV2Atom
}

pub unsafe fn lv2_atom_object_get(body: *mut LV2AtomObject, query: &[ObjectHelper]) -> i32 {
    
    let mut matches = 0;
    let mut n_queries = 0;

    for it in query {
        if it.atom.is_null() {
            return -1;
        }
        n_queries += 1;
    }

    {
        let f = | prop: *mut LV2AtomPropertyBody| -> bool {

                for it in query {
                    let qkey = it.key;

                    if qkey == (*prop).key && (*(it.atom)).is_null() {
                        *(it.atom) = &mut (*prop).value;
                        matches += 1;
                        if matches == n_queries {
                            return matches > 0;
                        }
                        break;
                    }
                }
                return true;
            };

        (*body).foreach(f);
    }

    return matches;
}
