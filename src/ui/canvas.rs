#[derive(Clone, Copy)]
struct Cell {
    content: char
}

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

        for _ in 0..width*height {
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

    pub fn render(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get_cell_content(x, y));
            }

            println!();
        }
    }
}
