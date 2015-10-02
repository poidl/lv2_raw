killall jalv
sleep 0.2
jalv http://example.org/brrrsynth &
sleep 0.2
jack_connect "Little synth:in"  "a2j:Keystation 49 [28] (capture): Keystation 49 MIDI 1"
jack_connect "Little synth:out" "system:playback_1"
jack_connect "Little synth:out" "system:playback_2"
