mod file_check;

fn main() {
    // print!("{esc}c", esc = 27 as char); // Clear the terminal screen.
    println!("Hello World!");
    file_check::ensure_notes("keys");
}
