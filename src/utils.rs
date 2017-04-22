// Copyright 2017 Michael Oswald

// Documentation copied from http://lv2plug.in/ns/ext/atom/util.h

// Copyright text of the original C file:

// Copyright 2012-2016 David Robillard <http://drobilla.net>
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



use atom::*;
use std::ptr;
use std::mem::size_of;
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
    (a == b) ||
    (((*a).mytype == (*b).mytype) && ((*a).size == (*b).size) &&
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
                                    size: u32)
                                    -> *const LV2AtomEvent {

    (body as *const u8).offset(lv2_atom_pad_size(size) as isize) as *const LV2AtomEvent
}

/** Return true iff `i` has reached the end of `body`. */
pub unsafe fn lv2_atom_sequence_is_end(body: *const LV2AtomSequenceBody,
                                       size: u32,
                                       i: *const LV2AtomEvent)
                                       -> bool {

    let result = i as *const u8 >= (body as *const u8).offset(size as isize);
    result
}


/** Return an iterator to the element following `i`. */
pub unsafe fn lv2_atom_sequence_next(i: *const LV2AtomEvent) -> *mut LV2AtomEvent {
    let off = size_of::<LV2AtomEvent>() + lv2_atom_pad_size((*i).body.size) as usize;
    let ptr = (i as *const u8).offset(off as isize);

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
                                             capacity: u32,
                                             event: *const LV2AtomEvent)
                                             -> *const LV2AtomEvent {

    let total_size = size_of::<LV2AtomEvent>() as u32 + (*event).body.size;

    if (capacity - (*seq).atom.size) < total_size {
        return 0 as *const LV2AtomEvent;
    }

    let e = lv2_atom_sequence_end(&(*seq).body, (*seq).atom.size);
    memcpy(e as *mut c_void,
           event as *const c_void,
           total_size as usize);

    (*seq).atom.size += lv2_atom_pad_size(total_size);

    e
}

/** Return a pointer to the first property in `body`. */
pub unsafe fn lv2_atom_object_begin(body: *const LV2AtomObjectBody) -> *mut LV2AtomPropertyBody {

    body.offset(1) as *mut LV2AtomPropertyBody
}

/** Return true iff `i` has reached the end of `obj`. */
pub unsafe fn lv2_atom_object_is_end(body: *const LV2AtomObjectBody,
                                     size: u32,
                                     i: *const LV2AtomPropertyBody)
                                     -> bool {

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
    pub value: *mut *mut LV2Atom,
}

/**
   Get an object's values for various keys.

   The value pointer of each item in `query` will be set to the location of
   the corresponding value in `object`.  Every value pointer in `query` MUST
   be initialised to NULL.  This function reads `object` in a single linear
   sweep.  By allocating `query` on the stack, objects can be "queried"
   quickly without allocating any memory.  This function is realtime safe.

   This function can only do "flat" queries, it is not smart enough to match
   variables in nested objects.

   For example:
   @code
   const LV2_Atom* name = NULL;
   const LV2_Atom* age  = NULL;
   LV2_Atom_Object_Query q[] = {
       { urids.eg_name, &name },
       { urids.eg_age,  &age },
       LV2_ATOM_OBJECT_QUERY_END
   };
   lv2_atom_object_query(obj, q);
   // name and age are now set to the appropriate values in obj, or NULL.
   @endcode
*/
pub unsafe fn lv2_atom_object_query(obj: *mut LV2AtomObject,
                                    query: *mut LV2AtomObjectQuery)
                                    -> i32 {

    let ref mut object = *obj;

    let mut n_queries = 0;
    let mut matches = 0;

    let q = query;
    while (*q).key != 0 {
        n_queries += 1;
        q.offset(1);
    }

    {
        let f = |prop: *mut LV2AtomPropertyBody| -> bool {
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
    pub atom: *mut *mut LV2Atom,
}

/**
   Variable argument version of lv2_atom_object_query().

   This is nicer-looking in code, but a bit more error-prone since it is not
   type safe and the argument list must be terminated.

   The arguments should be a series of uint32_t key and const LV2_Atom** value
   pairs, terminated by a zero key.  The value pointers MUST be initialized to
   NULL.  For example:

   @code
   const LV2_Atom* name = NULL;
   const LV2_Atom* age  = NULL;
   lv2_atom_object_get(obj,
                       uris.name_key, &name,
                       uris.age_key,  &age,
                       0);
   @endcode
*/
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
        let f = |prop: *mut LV2AtomPropertyBody| -> bool {

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

impl LV2AtomSequenceBody {
    pub unsafe fn foreach<F>(&mut self, size: u32, mut closure: F) -> ()
        where F: FnMut(*const LV2AtomEvent) -> ()
    {

        let mut it = lv2_atom_sequence_begin(self);
        while !lv2_atom_sequence_is_end(self, size, it) {
            closure(it);
            it = lv2_atom_sequence_next(it);
        }
    }
}

pub struct LV2AtomSequenceIterator<'a> {
    pub seq: &'a LV2AtomSequence,
    pub current: &'a LV2AtomEvent,
}

impl<'a> Iterator for LV2AtomSequenceIterator<'a> {
    type Item = &'a LV2AtomEvent;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let body = &self.seq.body;
            let size = self.seq.atom.size;
            if !lv2_atom_sequence_is_end(body, size, self.current) {
                let out = self.current;
                self.current = &*lv2_atom_sequence_next(self.current);
                Some(out)
            } else {
                None
            }
        }
    }
}

// perhaps wrong. TODO: understand this: http://stackoverflow.com/questions/41448232/issues-constraining-implementation-lifetimes-on-type-without-lifetime-parameter
impl<'a> IntoIterator for &'a LV2AtomSequence {
    type Item = &'a LV2AtomEvent;
    type IntoIter = LV2AtomSequenceIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            LV2AtomSequenceIterator {
                seq: &*self,
                current: &*lv2_atom_sequence_begin(&(*self).body),
            }
        }
    }
}
