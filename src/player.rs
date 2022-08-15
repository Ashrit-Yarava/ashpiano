use console::Term;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;

// Declare global variables for the current octave.
static LEFT_OCTAVE: i32 = 4;
static RIGHT_OCTAVE: i32 = 5;

fn octave_info() {
    println!("Left Octave: {}", LEFT_OCTAVE);
    println!("Right Octave: {}", RIGHT_OCTAVE);
}

fn keyboard_event_loop() {
    let stdout = Term::buffered_stdout();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                // Left octave keys
                'q' => play_sound(&stream_handle, LEFT_OCTAVE, "c"),
                'w' => play_sound(&stream_handle, LEFT_OCTAVE, "d"),
                'e' => play_sound(&stream_handle, LEFT_OCTAVE, "e"),
                'r' => play_sound(&stream_handle, LEFT_OCTAVE, "f"),
                't' => play_sound(&stream_handle, LEFT_OCTAVE, "g"),
                'y' => play_sound(&stream_handle, LEFT_OCTAVE, "a"),
                'u' => play_sound(&stream_handle, LEFT_OCTAVE, "b"),

                // Right octave keys
                'v' => play_sound(&stream_handle, RIGHT_OCTAVE, "c"),
                'b' => play_sound(&stream_handle, RIGHT_OCTAVE, "d"),
                'n' => play_sound(&stream_handle, RIGHT_OCTAVE, "e"),
                'm' => play_sound(&stream_handle, RIGHT_OCTAVE, "f"),
                ',' => play_sound(&stream_handle, RIGHT_OCTAVE, "g"),
                '.' => play_sound(&stream_handle, RIGHT_OCTAVE, "a"),
                '/' => play_sound(&stream_handle, RIGHT_OCTAVE, "b"),
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
    octave_info();
    keyboard_event_loop();
}
