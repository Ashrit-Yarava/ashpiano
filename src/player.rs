mod play_sound;
use console::Term;

use self::play_sound::play_sound;

// Declare global variables for the current octave.
static LEFT_OCTAVE: i32 = 4;
static RIGHT_OCTAVE: i32 = 5;

fn octave_info() {
    println!("Left Octave: {}", LEFT_OCTAVE);
    println!("Right Octave: {}", RIGHT_OCTAVE);
}

pub fn player() {
    octave_info();

    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                'q' => play_sound("c", LEFT_OCTAVE),
                'w' => println!("Up"),
                'a' => println!("Left"),
                's' => println!("Down"),
                'd' => println!("Right"),
                _ => break,
            }
        }
    }
}
