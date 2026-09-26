use std::io::{self, Write};

pub fn flush_output() {
    io::stdout().flush().unwrap();
}

pub fn move_cursor(x: usize, y: usize) {
    print!("\x1B[{};{}H", y + 1, x + 1);
}
