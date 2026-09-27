use std::io::{self, Write};

pub fn flush_output() {
    io::stdout().flush().unwrap();
}

pub fn move_cursor(x: usize, y: usize) {
    print!("\x1B[{};{}H", y + 1, x + 1);
}

fn color_to_rgb(color: u32) -> (u32, u32, u32) {
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;

    (r, g, b)
}

pub fn set_fg(color: u32) {
    let (r, g, b) = color_to_rgb(color);
    print!("\x1B[38;2;{};{};{}m", r, g, b);
}

pub fn set_bg(color: u32) {
    let (r, g, b) = color_to_rgb(color);
    print!("\x1B[48;2;{};{};{}m", r, g, b);
}
