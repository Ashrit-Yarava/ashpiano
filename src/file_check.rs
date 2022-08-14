// file_check.rs
// ---
// Ensure that the file tree for the piano recordings is present and in order.
//
// Sample File Structure
//
// notes/
// => octave_0/
//  => a.mp3
//  => a-sharp.mp3
//  => b.mp3
// => octave_1/
//  => c.mp3
//  => c-sharp.mp3
//  => d.mp3
//  => d-sharp.mp3
//  => e.mp3
//  => f.mp3
//  => f-sharp.mp3
//  => g.mp3
//  => g-sharp.mp3
//  => a.mp3
//  => a-sharp.mp3
//  => b.mp3
// => octave_2/
//  => (Another set)
// => octave_3/
// => octave_4/
// => octave_5/
// => octave_6/
// => octave_7/
// => octave_8/
//
// Refer to image in the README for a visual of the individual keys.

use std::path::Path;

fn file_not_found() {}

pub fn ensure_notes() {}
