mod piece;
mod board;
mod coord;

fn main() {
    println!("dama rust rewrite, v0.1");
    let mut board: board::Board = board::Board::init();
    piece::move_to(&mut board.pieces[14].clone(), 20 as u8, &mut board);
    let moves: Option<Vec<u8>> = piece::possible_moves(&board.pieces[14], &board.tiles);
    match moves {
        Some(vec) => {
            for i in vec {
                println!("possible move: {i}");
            }
        }
        _ => { 
            println!("no move avaliable");
        }
    }
    println!("Thanks for playing dama ;)");
}
