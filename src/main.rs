mod piece;
mod board;
mod coord;
mod tiles;
mod capture;

fn main() {
    println!("dama rust rewrite, v0.1");
    let mut board: board::Board = board::Board::init();
    match piece::from_n(23, &mut board.clone()) {
        Some(piece) => {
            let mut moves: Option<Vec<u8>> = piece::possible_moves(piece, &board.tiles);
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
            piece::move_to(piece, 19, &mut board);
            moves = piece::possible_moves(piece, &board.tiles);
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
        }
        _ => { }
    }
    
    println!("Thanks for playing dama ;)");
}
