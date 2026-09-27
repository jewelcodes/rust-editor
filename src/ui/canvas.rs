use crate::ui::terminal;

const DEFAULT_BG: u32 = 0x000000;
const DEFAULT_FG: u32 = 0xD0D0D0;

#[derive(Clone, Copy)]
struct Cell {
    content: char,
    fg_color: u32,
    bg_color: u32
}

pub enum BorderType {
    None,
    Normal,
    Rounded
}

struct BorderCharacters {
    top: char,
    right: char,
    bottom: char,
    left: char,
    top_left: char,
    top_right: char,
    bottom_right: char,
    bottom_left: char
}

const BORDER_NONE: BorderCharacters = BorderCharacters {
    top: ' ',
    bottom: ' ',
    left: ' ',
    right: ' ',
    top_left: ' ',
    top_right: ' ',
    bottom_right: ' ',
    bottom_left: ' '
};

const BORDER_NORMAL: BorderCharacters = BorderCharacters {
    top: '─',
    bottom: '─',
    left: '│',
    right: '│',
    top_left: '┌',
    top_right: '┐',
    bottom_right: '┘',
    bottom_left: '└'
};

const BORDER_ROUNDED: BorderCharacters = BorderCharacters {
    top: '─',
    bottom: '─',
    left: '│',
    right: '│',
    top_left: '╭',
    top_right: '╮',
    bottom_right: '╯',
    bottom_left: '╰'
};

pub struct Canvas {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,

    cells: Vec<Cell>
}

impl Canvas {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        let mut canvas = Self {
            x, y, width, height,
            cells: Vec::new()
        };

        for _ in 0..width * height {
            canvas.cells.push(Cell {
                content: ' ',
                fg_color: DEFAULT_FG,
                bg_color: DEFAULT_BG
            });
        }

        canvas
    }

    pub fn get_cell_content(&self, x: usize, y: usize) -> char {
        self.cells[y * self.width + x].content
    }

    pub fn set_cell_content(&mut self, x: usize, y: usize, content: char) {
        self.cells[y * self.width + x].content = content;
    }

    pub fn get_cell_fg(&self, x: usize, y: usize) -> u32 {
        self.cells[y * self.width + x].fg_color
    }

    pub fn get_cell_bg(&self, x: usize, y: usize) -> u32 {
        self.cells[y * self.width + x].bg_color
    }

    pub fn set_cell_fg(&mut self, x: usize, y: usize, color: u32) {
        self.cells[y * self.width + x].fg_color = color;
    }

    pub fn set_cell_bg(&mut self, x: usize, y: usize, color: u32) {
        self.cells[y * self.width + x].bg_color = color;
    }

    pub fn set_border(&mut self, border_type: BorderType) {
        let border = match border_type {
            BorderType::Normal => &BORDER_NORMAL,
            BorderType::Rounded => &BORDER_ROUNDED,
            _ => &BORDER_NONE
        };

        self.set_cell_content(0, 0, border.top_left);
        self.set_cell_content(self.width - 1, 0, border.top_right);
        self.set_cell_content(self.width - 1, self.height - 1, border.bottom_right);
        self.set_cell_content(0, self.height - 1, border.bottom_left);

        for x in 1..self.width - 1 {
            self.set_cell_content(x, 0, border.top);
            self.set_cell_content(x, self.height - 1, border.bottom);
        }

        for y in 1..self.height - 1 {
            self.set_cell_content(0, y, border.left);
            self.set_cell_content(self.width - 1, y, border.right);
        }
    }

    pub fn set_border_color(&mut self, fg_color: u32, bg_color: u32) {
        for x in 0..self.width {
            self.set_cell_fg(x, 0, fg_color);
            self.set_cell_bg(x, 0, bg_color);
            self.set_cell_fg(x, self.height - 1, fg_color);
            self.set_cell_bg(x, self.height - 1, bg_color);
        }

        for y in 1..self.height - 1 {
            self.set_cell_fg(0, y, fg_color);
            self.set_cell_bg(0, y, bg_color);
            self.set_cell_fg(self.width - 1, y, fg_color);
            self.set_cell_bg(self.width - 1, y, bg_color);
        }
    }

    pub fn render(&self) {
        let mut current_fg = self.get_cell_fg(0, 0);
        let mut current_bg = self.get_cell_bg(0, 0);
        let mut next_fg;
        let mut next_bg;

        terminal::set_fg(current_fg);
        terminal::set_bg(current_bg);

        for y in 0..self.height {
            terminal::move_cursor(self.x, self.y + y);

            for x in 0..self.width {
                next_fg = self.get_cell_fg(x, y);
                if next_fg != current_fg {
                    current_fg = next_fg;
                    terminal::set_fg(current_fg);
                }

                next_bg = self.get_cell_bg(x, y);
                if next_bg != current_bg {
                    current_bg = next_bg;
                    terminal::set_bg(current_bg);
                }

                print!("{}", self.get_cell_content(x, y));
            }
        }

        terminal::flush_output();
    }
}
