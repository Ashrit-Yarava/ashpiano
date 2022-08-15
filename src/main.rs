mod file_check;
mod player;

fn main() {
    // print!("{esc}c", esc = 27 as char); // Clear the terminal screen.
    file_check::ensure_notes("keys");
    player::player();
}
