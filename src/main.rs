mod ui {
    pub mod canvas;
    pub mod terminal;
}

use ui::canvas::{Canvas, BorderType};

fn main() {
    let mut canvas = Canvas::new(0, 0, 40, 21);

    canvas.set_cell_content(14, 10, 'H');
    canvas.set_cell_content(15, 10, 'e');
    canvas.set_cell_content(16, 10, 'l');
    canvas.set_cell_content(17, 10, 'l');
    canvas.set_cell_content(18, 10, 'o');

    canvas.set_cell_content(20, 10, 'w');
    canvas.set_cell_content(21, 10, 'o');
    canvas.set_cell_content(22, 10, 'r');
    canvas.set_cell_content(23, 10, 'l');
    canvas.set_cell_content(24, 10, 'd');
    canvas.set_cell_content(25, 10, '!');

    canvas.set_border(BorderType::Rounded);
    canvas.render();

    loop {}
}
