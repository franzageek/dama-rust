mod piece;
mod board;
mod coord;

fn main() {
    println!("dama rust rewrite, v0.1");
    let board: board::Board = board::Board::init();
    let moves: Option<Vec<u8>> = piece::possible_moves(&board.pieces[14], &board.tiles);
    match moves {
        Some(vec) => {
            for i in vec {
                println!("possible move: {i}");
            }
        }
        _ => {}
    }
    println!("Thanks for playing dama ;)");
}
