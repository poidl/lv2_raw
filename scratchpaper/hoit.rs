pub extern fn run(instance: lv2::Lv2handle, n_samples: u32) {
    unsafe{
        let synth = instance as *mut Lv2SynthPlugin;
        let uris = &mut (*synth).uris;
        let seq = (*synth).in_port;
        let output = (*synth).output;
        // pointer to 1st event body
        let mut ev: *const lv2::Lv2AtomEvent  = lv2::lv2_atom_sequence_begin(&(*seq).body);
        // loop through event sequence
        while !lv2::lv2_atom_sequence_is_end(&(*seq).body, (*seq).atom.size, ev) {
            // check if event is midi
            if (*ev).body.mytype == (*uris).midi_event {

                // pointer to midi event data
                let msg: *const u8 = ev.offset(1) as *const u8;
                (*synth).midievent(msg);
                for i in istart-1..n_samples {
                    *output.offset(i as isize) = (*synth).getAmp();
                }
            }
            ev = lv2::lv2_atom_sequence_next(ev);
        }
    }
}

pub extern fn run(instance: lv2::Lv2handle, n_samples: u32) {
    unsafe{
        let synth = instance as *mut Synth;

                // frameindex of eventstart. In jalv this is relative to currently processed buffer chunk of length n_samples
                let istart = (*ev).time_in_frames as u32;

                match lv2::lv2_midi_message_type(msg) {

                    // note on event
                    lv2::Lv2MidiMessageType::Lv2MidiMsgNoteOn => {
                        (*synth).noteison = true;
                        let f0 = f0_from_midimsg(msg);
                        (*synth).f0 = f0;
                        (*synth).currentmidivel = *msg.offset(2);
                        let coef = 1.0 as f32;

                        (*synth).osc.reset();
                        (*synth).osc.set_dphase(f0,(*synth).fs);

                        // TODO don't set fs here
                        (*synth).oscST.reset((*synth).fs);
                        (*synth).oscST.set_f0fn(f0);

                        for i in istart-1..n_samples {
                            // let amp = (*synth).osc.get() as f32;
                            let amp = (*synth).oscST.get() as f32;
                            *output.offset(i as isize) = amp;
                        }
                    }

                    // note off event
                    lv2::Lv2MidiMessageType::Lv2MidiMsgNoteOff => {
                        (*synth).noteison = false;
                        (*synth).makesilence = true;
                        for i in istart-1..n_samples {
                            let amp = 0.0 as f32;
                            *output.offset(i as isize) = amp as f32;
                        }
                    }

                    _ => {
                        println!("DON'T UNDERSTAND MESSAGE")
                    }

                }
            }
            ev = lv2::lv2_atom_sequence_next(ev);
        }

        if (*synth).noteison {
            let coef = 1.0 as f32;
            let f0 = (*synth).f0;

            for i in 0..n_samples {
                // let amp = (*synth).osc.get();
                let amp = (*synth).oscST.get();
                *output.offset(i as isize) = (amp as f32) * coef;
            }

        } else if (*synth).makesilence {
            (*synth).makesilence = false;
            for i in 0..n_samples {
                let amp = 0.0;
                *output.offset(i as isize) = amp as f32;
            }
        }

    }

}
