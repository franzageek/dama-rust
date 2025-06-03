use crate::coord;

#[derive(Clone)]
pub struct Piece {
    pub n: u8,
    pub player: bool,
    pub king: bool,
    pub valid: bool
}

enum Pos {
    None = 0,
    BottomLeft = 1,
    BottomRight = 2,
    TopRight = 3,
    TopLeft = 4
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

fn get_next(pos: Pos, n: u8) -> u8 {
    let (_, y): (u8, u8) =  coord::xy_from_n(n);
    match pos {
        Pos::None => { 
            return 0; 
        }
        Pos::BottomLeft => {
            return if n != 1 && n != 9 && n != 17 && n != 25 {
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
            return if n != 8 && n != 16 && n != 24 && n != 32 {
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
            return if n != 8 && n != 16 && n != 24 && n != 32 {
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
            return if n != 1 && n != 9 && n != 17 && n != 25 {
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
}

fn check_next(next: u8, tiles: &Vec<u8>) -> bool {
    return next > 0 && next < 32 && tiles[(next-1) as usize] == 0;
}

pub fn possible_moves(piece: &Piece, tiles: &Vec<u8>) -> Option<Vec<u8>> {
    let mut vec = Vec::new();
    if !piece.king {
        if piece.player {
            let mut next: u8 = get_next(Pos::BottomLeft, piece.n);
            if check_next(next, tiles) {
                vec.push(next);
            } 
            next = get_next(Pos::BottomRight, piece.n);
            if check_next(next, tiles) {
                vec.push(next);
            } 
        } else {
            let mut next: u8 = get_next(Pos::TopRight, piece.n);
            if check_next(next, tiles) {
                vec.push(next);
            } 
            next = get_next(Pos::TopLeft, piece.n);
            if check_next(next, tiles) {
                vec.push(next);
            } 
        }
    } else {
        let mut next: u8 = get_next(Pos::BottomLeft, piece.n);
        if check_next(next, tiles) {
            vec.push(next);
        } 
        next = get_next(Pos::BottomRight, piece.n);
        if check_next(next, tiles) {
            vec.push(next);
        } 
        let mut next: u8 = get_next(Pos::TopRight, piece.n);
        if check_next(next, tiles) {
            vec.push(next);
        } 
        next = get_next(Pos::TopLeft, piece.n);
        if check_next(next, tiles) {
            vec.push(next);
        } 
    }
    return if vec.len() > 0 {
        Some(vec)
    } else {
        None
    };
}