static LEFT_OCTAVE: i32 = 4;
static RIGHT_OCTAVE: i32 = 5;

fn octave_info() {
    println!("Left Octave: {}", LEFT_OCTAVE);
    println!("Right Octave: {}", RIGHT_OCTAVE);
}

pub fn player() {
    octave_info();
}
