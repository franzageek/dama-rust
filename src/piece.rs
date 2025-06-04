use crate::coord;
use crate::board::*;
#[allow(dead_code)]
#[derive(Clone)]
pub struct Piece {
    pub n: u8,
    pub player: bool,
    pub king: bool,
    pub valid: bool
}

#[allow(dead_code)]
enum Pos {
    None = 0,
    BottomLeft = 1,
    BottomRight = 2,
    TopRight = 3,
    TopLeft = 4
}

#[allow(dead_code)]
pub enum TileState {
    Free(u8),
    Busy(u8),
    OutOfRange
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

fn get_next(pos: Pos, n: u8, tiles: &Vec<u8>) -> TileState {
    let (_, y): (u8, u8) =  coord::xy_from_n(n);
    let next: u8;
    match pos {
        Pos::None => { 
            next = 0; 
        }
        Pos::BottomLeft => {
            next = if n != 1 && n != 9 && n != 17 && n != 25 {
                n + if y % 2 == 0 { 
                    3 
                } else {
                    4
                }
            } else {
                0
            };
        }
        Pos::BottomRight => {
            next =  if n != 8 && n != 16 && n != 24 && n != 32 {
                n + if y % 2 == 0 {
                    4
                } else {
                    5
                }
            } else {
                0
            };
        }
        Pos::TopRight => {
            next = if n != 8 && n != 16 && n != 24 && n != 32 {
                n - if y % 2 == 0 { 
                    4 
                } else {
                    3
                }
            } else {
                0
            };
        }
        Pos::TopLeft => {
            next = if n != 1 && n != 9 && n != 17 && n != 25 {
                n - if y % 2 == 0 { 
                    5
                } else {
                    4
                }
            } else {
                0
            };
        }
    }
    if next > 0 && next < 32 {
        if tiles[(next-1) as usize] == 0 {
            return TileState::Free(next);
        } else {
            return TileState::Busy(next);
        }
    } else {
        return TileState::OutOfRange;
    }
}

pub fn possible_moves(piece: &Piece, tiles: &Vec<u8>) -> Option<Vec<u8>> {
    let mut vec = Vec::new();
    if !piece.king {
        if piece.player {
            let mut next: TileState = get_next(Pos::BottomLeft, piece.n, tiles);
            match next {
                TileState::Free(n) => {
                    vec.push(n);
                }
                _ => {}
            }
            next = get_next(Pos::BottomRight, piece.n, tiles);
            match next {
                TileState::Free(n) => {
                    vec.push(n);
                }
                _ => {}
            }
        } else {
            let mut next: TileState = get_next(Pos::TopRight, piece.n, tiles);
            match next {
                TileState::Free(n) => {
                    vec.push(n);
                }
                _ => {}
            }
            next = get_next(Pos::TopLeft, piece.n, tiles);
            match next {
                TileState::Free(n) => {
                    vec.push(n);
                }
                _ => {}
            } 
        }
    } else {
        let mut next: TileState = get_next(Pos::BottomLeft, piece.n, tiles);
        match next {
            TileState::Free(n) => {
                vec.push(n);
            }
            _ => {}
        }
        next = get_next(Pos::BottomRight, piece.n, tiles);
        match next {
            TileState::Free(n) => {
                vec.push(n);
            }
            _ => {}
        } 
        next = get_next(Pos::TopRight, piece.n, tiles);
        match next {
            TileState::Free(n) => {
                vec.push(n);
            }
            _ => {}
        } 
        next = get_next(Pos::TopLeft, piece.n, tiles);
        match next {
            TileState::Free(n) => {
                vec.push(n);
            }
            _ => {}
        } 
    }
    return if vec.len() > 0 {
        Some(vec)
    } else {
        None
    };
}

pub fn move_to(piece: &mut Piece, n: u8, board: &mut Board) {
    let index: u8 = board.tiles[(piece.n-1) as usize];
    board.tiles[(n-1) as usize] = index;
    board.tiles[(piece.n-1) as usize] = 0;
    piece.n = n;
    return;
}

pub fn from_n(n: u8, board: &mut Board) -> Option<&mut Piece> {
    return if n > 0 && n <= 32 {
        Some(&mut board.pieces[(board.tiles[(n-1) as usize]-1) as usize])
    } else {
        None
    };
}