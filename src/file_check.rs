// file_check.rs
// ---
// Ensure that the file tree for the piano recordings is present and in order.
//
// Sample File Structure
//
// notes/
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
//
// Refer to image in the README for a visual of the individual keys.

extern crate exitcode;
use std::path::Path;
use std::process;

fn file_not_found(file: String) {
    println!("FILE or DIRECTORY not found. Error File: {}", file);
    process::exit(exitcode::OSFILE); // OS File error indicates a file was not found on the
                                     // operating system.
}

fn check_dir(file: &str) {
    if !Path::new(file).is_dir() {
        file_not_found(file.to_string());
    }
}

fn check_file(file: &str) {
    if !Path::new(file).is_file() {
        file_not_found(file.to_string());
    }
}

pub fn ensure_notes(root_dir: &str) {
    // Check the top level directory.

    check_dir(root_dir);

    // The 7 primary octaves.
    let octaves: [&str; 7] = [
        "octave_1", "octave_2", "octave_3", "octave_4", "octave_5", "octave_6", "octave_7",
    ];

    for octave in octaves {
        // Check the directory exists, then each of the files in it.
        check_dir(&format!("{}/{}/", root_dir, octave));

        let keys: [&str; 12] = [
            "c", "c-sharp", "d", "d-sharp", "e", "f", "f-sharp", "g", "g-sharp", "a", "a-sharp",
            "b",
        ];

        for file in keys {
            check_file(&format!("{}/{}/{}.wav", root_dir, octave, file));
        }
    }
}
