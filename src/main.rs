use midly::{EventKind, MidiMessage, Smf};
use std::collections::BTreeMap;
use std::fs;

#[derive(Debug)]
struct MidiNote {
    channel: u8,
    note: u8,
}

fn main() {
    println!("Hello, world!");

    let data = fs::read("bwv542.mid").unwrap();
    let smf = Smf::parse(&data).unwrap();

    // Fill B-Trees for note events

    let mut note_on: BTreeMap<u32, Vec<MidiNote>> = BTreeMap::new();
    let mut note_off: BTreeMap<u32, Vec<MidiNote>> = BTreeMap::new();

    for event in smf.tracks.iter().flat_map(|track| track.iter()) {
        match event.kind {
            EventKind::Midi { channel, message } => match message {
                MidiMessage::NoteOff { key, vel: _ } => {
                    note_off
                        .entry(event.delta.as_int())
                        .or_insert(Vec::new())
                        .push(MidiNote {
                            channel: channel.as_int(),
                            note: key.as_int(),
                        });
                }
                MidiMessage::NoteOn { key, vel: _ } => {
                    note_on
                        .entry(event.delta.as_int())
                        .or_insert(Vec::new())
                        .push(MidiNote {
                            channel: channel.as_int(),
                            note: key.as_int(),
                        });
                }
                _ => {}
            },
            _ => {}
        }
    }

    // calculate quatization
    let min_time = note_on.iter().next().unwrap().0;
    let max_time = note_off.iter().next_back().unwrap().0;
    println!("{}, {}", min_time, max_time);

    // do quantization
    let mut current: Vec<u8> = Vec::new();
    // -> iterate over quantization time windows and turn notes on/off, then write row
    // -> including channels and note or only note can be write option
}
