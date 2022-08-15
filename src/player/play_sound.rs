use play;

pub fn play_sound(note: &str, octave: i32) {
    let audio_file = format!("keys/octave_{}/{}.mp3", octave, note);

    play::play(audio_file).unwrap();
}
