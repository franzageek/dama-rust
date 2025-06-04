use crate::piece::Piece;

mod piece;
mod board;
mod coord;

fn main() {
    println!("dama rust rewrite, v0.1");
    let mut board: board::Board = board::Board::init();
    let mut piece: Piece = board.pieces[14].clone();
    piece::move_to(&mut piece, 20 as u8, &mut board);
    let moves: Option<Vec<u8>> = piece::possible_moves(&piece, &board.tiles);
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
