use console::Term;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;

fn octave_info(left_octave: i32, right_octave: i32) {
    println!("Left Octave: {}", left_octave);
    println!("Right Octave: {}", right_octave);
}

fn keyboard_event_loop() {
    let stdout = Term::buffered_stdout();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut left_octave: i32 = 4;
    let mut right_octave: i32 = 5;

    loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                // Left octave keys
                'q' => play_sound(&stream_handle, left_octave, "c"),
                '2' => play_sound(&stream_handle, left_octave, "c-sharp"),
                'w' => play_sound(&stream_handle, left_octave, "d"),
                '3' => play_sound(&stream_handle, left_octave, "d-sharp"),
                'e' => play_sound(&stream_handle, left_octave, "e"),
                'r' => play_sound(&stream_handle, left_octave, "f"),
                '5' => play_sound(&stream_handle, left_octave, "f-sharp"),
                't' => play_sound(&stream_handle, left_octave, "g"),
                '6' => play_sound(&stream_handle, left_octave, "g-sharp"),
                'y' => play_sound(&stream_handle, left_octave, "a"),
                '7' => play_sound(&stream_handle, left_octave, "a-sharp"),
                'u' => play_sound(&stream_handle, left_octave, "b"),

                // Changing octaves.
                'a' => left_octave = 1,
                's' => left_octave = 2,
                'd' => left_octave = 3,

                // Right octave keys
                'v' => play_sound(&stream_handle, right_octave, "c"),
                'g' => play_sound(&stream_handle, right_octave, "c-sharp"),
                'b' => play_sound(&stream_handle, right_octave, "d"),
                'h' => play_sound(&stream_handle, right_octave, "d-sharp"),
                'n' => play_sound(&stream_handle, right_octave, "e"),
                'm' => play_sound(&stream_handle, right_octave, "f"),
                'k' => play_sound(&stream_handle, right_octave, "f-sharp"),
                ',' => play_sound(&stream_handle, right_octave, "g"),
                'l' => play_sound(&stream_handle, right_octave, "g-sharp"),
                '.' => play_sound(&stream_handle, right_octave, "a"),
                ';' => play_sound(&stream_handle, right_octave, "a-sharp"),
                '/' => play_sound(&stream_handle, right_octave, "b"),

                // Changing octaves.
                'i' => right_octave = 4,
                'o' => right_octave = 5,
                'p' => right_octave = 6,
                '[' => right_octave = 7,

                // Print which octaves are currently in use.
                '\\' => octave_info(left_octave, right_octave),
                _ => break,
            }
        }
    }
}

fn play_sound(stream_handle: &OutputStreamHandle, octave: i32, note: &str) {
    let sink = Sink::try_new(stream_handle).unwrap();
    let file = BufReader::new(File::open(format!("keys/octave_{}/{}.wav", octave, note)).unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    sink.detach(); // Destroy the sink since it's not needed anymore. The sound will keep playing.
}

pub fn player() {
    keyboard_event_loop();
}
