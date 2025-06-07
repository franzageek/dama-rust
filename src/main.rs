mod board;
mod capture;
mod coord;
mod game;
mod piece;
mod tiles;
mod ui;

use raylib;

fn main() {
    println!("dama rust rewrite, v0.1");
    let mut board: board::Board = board::Board::init();
    game::main_loop(
        raylib::init()
            .size(ui::WINDOW_SIZE as i32, ui::WINDOW_SIZE as i32)
            .title("dama v0.1")
            .build(),
        &mut board,
    );
    println!("Thanks for playing dama ;)");
}
