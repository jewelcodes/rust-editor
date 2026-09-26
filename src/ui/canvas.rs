use crate::ui::terminal;

#[derive(Clone, Copy)]
struct Cell {
    content: char
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
                content: ' '
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

    pub fn render(&self) {
        for y in 0..self.height {
            terminal::move_cursor(self.x, self.y + y);

            for x in 0..self.width {
                print!("{}", self.get_cell_content(x, y));
            }
        }

        terminal::flush_output();
    }
}
