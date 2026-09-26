mod ui {
    pub mod canvas;
}

fn main() {
    let mut canvas = ui::canvas::Canvas::new(0, 0, 20, 15);

    canvas.set_cell_content(4, 7, 'H');
    canvas.set_cell_content(5, 7, 'e');
    canvas.set_cell_content(6, 7, 'l');
    canvas.set_cell_content(7, 7, 'l');
    canvas.set_cell_content(8, 7, 'o');

    canvas.set_cell_content(10, 7, 'w');
    canvas.set_cell_content(11, 7, 'o');
    canvas.set_cell_content(12, 7, 'r');
    canvas.set_cell_content(13, 7, 'l');
    canvas.set_cell_content(14, 7, 'd');
    canvas.set_cell_content(15, 7, '!');

    canvas.render();
}
