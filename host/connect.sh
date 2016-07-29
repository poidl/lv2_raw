killall yassyhost
sleep 0.2
./target/debug/yassyhost &
sleep 0.2
jack_connect "yassyhost:midi_in"  "a2j:Keystation 49 [24] (capture): Keystation 49 MIDI 1"
jack_connect "yassyhost:audio_out" "system:playback_1"
jack_connect "yassyhost:audio_out" "system:playback_2"
