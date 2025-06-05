mod piece;
mod board;
mod coord;
mod tiles;
mod capture;
mod ui;
mod game;

use raylib;

fn main() {
    println!("dama rust rewrite, v0.1");
    let mut board: board::Board = board::Board::init();
    game::main_loop(
        raylib::init()
        .size(
            ui::WINDOW_SIZE as i32, 
            ui::WINDOW_SIZE as i32
        ).title("dama v0.1")
        .build(),
        &mut board
    );
    match piece::from_n(23, &mut board.clone()) {
        Some(piece) => {
            let mut moves: Option<Vec<u8>> = piece::possible_moves(piece, &board.tiles);
            match moves {
                Some(vec) => {
                    for i in vec {
                        println!("possible move from 23: {i}");
                    }
                }
                _ => { 
                    println!("no move avaliable");
                }
            }
            piece::move_to(piece, 20, &mut board);
            moves = piece::possible_moves(piece, &board.tiles);
            match moves {
                Some(vec) => {
                    for i in vec {
                        println!("possible move from 20: {i}");
                    }
                }
                _ => { 
                    println!("no move avaliable");
                }
            }
            if let Some(piece2) = piece::from_n(12, &mut board.clone()) {
                moves = piece::possible_moves(piece2, &board.tiles);
                match moves {
                    Some(vec) => {
                        for i in vec {
                            println!("possible move from 12: {i}");
                        }
                    }
                    _ => { 
                        println!("no move avaliable");
                    }
                }
                piece::move_to(piece2, 16, &mut board);
                board.tiles[29] = 0;
                println!("{:?}", capture::get_possible(&piece2, &mut board, 0, tiles::Pos::None));
            }
        }
        _ => { }
    }
    
    println!("Thanks for playing dama ;)");
}
