mod banner;
mod file_check;
mod player;

fn main() {
    print!("{esc}c", esc = 27 as char); // Clear the terminal screen
    banner::print_banner();
    file_check::ensure_notes("keys");
    player::player();
}
