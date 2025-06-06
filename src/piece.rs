use crate::tiles::*;
use crate::board::*;
#[allow(dead_code)]
#[derive(Clone)]
pub struct Piece {
    pub n: u8,
    pub player: bool,
    pub king: bool,
    pub valid: bool
}

impl Piece {
    pub fn init() -> Vec<Piece> {
        print!("initializing pieces...");
        let mut vec: Vec<Piece> = Vec::new();
        for i in 0u8..24u8 {
            vec.push(
                Piece { 
                    n: i + if i < 12 {
                        1
                    } else {
                        9
                    }, 
                    player: if i < 12 {
                        true
                    } else {
                        false
                    }, 
                    king: false, 
                    valid: true 
                }
            );
        }
        print!("done\n");
        return vec;
    }
}

pub fn possible_moves(piece: &Piece, tiles: &Vec<u8>) -> Vec<u8> {
    let mut vec = Vec::with_capacity(0);
    if !piece.king {
        if piece.player {
            match get_next(Pos::BottomLeft, piece.n, tiles) {
                TileState::Free(n) => {
                    vec.push(n);
                }
                _ => {}
            }
            match get_next(Pos::BottomRight, piece.n, tiles) {
                TileState::Free(n) => {
                    vec.push(n);
                }
                _ => {}
            }
        } else {
            match get_next(Pos::TopRight, piece.n, tiles) {
                TileState::Free(n) => {
                    vec.push(n);
                }
                _ => {}
            }
            match get_next(Pos::TopLeft, piece.n, tiles) {
                TileState::Free(n) => {
                    vec.push(n);
                }
                _ => {}
            } 
        }
    } else {
        match get_next(Pos::BottomLeft, piece.n, tiles) {
            TileState::Free(n) => {
                vec.push(n);
            }
            _ => {}
        }
        match get_next(Pos::BottomRight, piece.n, tiles) {
            TileState::Free(n) => {
                vec.push(n);
            }
            _ => {}
        } 
        match get_next(Pos::TopRight, piece.n, tiles) {
            TileState::Free(n) => {
                vec.push(n);
            }
            _ => {}
        } 
        match get_next(Pos::TopLeft, piece.n, tiles) {
            TileState::Free(n) => {
                vec.push(n);
            }
            _ => {}
        } 
    }
    return vec;
}

pub fn move_to(nfrom: u8, nto: u8, board: &mut Board) {
    let index: u8 = board.tiles[(nfrom-1) as usize];
    board.tiles[(nto-1) as usize] = index;
    board.tiles[(nfrom-1) as usize] = 0;
    board.pieces[(index-1) as usize].n = nto;
    return;
}

pub fn from_n(n: u8, board: &mut Board) -> Option<&mut Piece> {
    return if n > 0 && n <= 32 && board.tiles[(n-1) as usize] != 0 {
        Some(&mut board.pieces[(board.tiles[(n-1) as usize]-1) as usize])
    } else {
        //println!("error: not a valid piece");
        None
    };
}