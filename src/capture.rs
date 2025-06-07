use crate::{
    board, piece,
    tiles::{self, TileState},
};

#[derive(Debug, Clone)]
pub struct Capture {
    pub ndest: u8,
    pub ncapture: u8,
    pub next: Vec<Box<Capture>>,
}

fn get_capture(
    piece: &mut piece::Piece,
    board: &mut board::Board,
    auxn: Option<u8>,
    pos: tiles::Pos,
    ivec: &Vec<Box<Capture>>,
) -> Vec<Box<Capture>> {
    let mut vec: Vec<Box<Capture>> = Vec::with_capacity(0);
    if ivec.len() > 0 {
        vec = ivec.clone();
    }
    match tiles::get_next(
        pos,
        match auxn {
            Some(aux_n) => aux_n,
            _ => piece.n,
        },
        &board.tiles,
    ) {
        tiles::TileState::Busy(n0) => {
            if let Some(piece0) = piece::from_n(n0, board) {
                if (piece0.king == piece.king || (piece.king && !piece0.king))
                    && (piece0.player != piece.player)
                {
                    match tiles::get_next(pos, n0, &board.tiles) {
                        TileState::Free(n1) => {
                            vec.push(Box::new(Capture {
                                ndest: n1,
                                ncapture: n0,
                                next: get_possible(Some(piece), board, n1, pos),
                            }));
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
    return vec;
}

pub fn get_possible(
    in_piece: Option<&mut piece::Piece>,
    board: &mut board::Board,
    auxn: u8,
    last_move: tiles::Pos,
) -> Vec<Box<Capture>> {
    let mut vec: Vec<Box<Capture>> = Vec::with_capacity(0);
    if let Some(piece) = in_piece {
        if piece.king {
            for i in 1u8..=4u8 {
                if last_move as u8 > 0 {
                    if i != ((last_move as u8 + 1) % 4) + 1 {
                        vec = get_capture(
                            piece,
                            board,
                            if auxn > 0 && auxn <= 32 {
                                Some(auxn)
                            } else {
                                None
                            },
                            tiles::Pos::from(i),
                            &vec,
                        ); //fix cast
                    }
                } else {
                    vec = get_capture(
                        piece,
                        board,
                        if auxn > 0 && auxn <= 32 {
                            Some(auxn)
                        } else {
                            None
                        },
                        tiles::Pos::from(i),
                        &vec,
                    );
                }
            }
        } else {
            if piece.player {
                vec = get_capture(
                    piece,
                    board,
                    if auxn > 0 && auxn <= 32 {
                        Some(auxn)
                    } else {
                        None
                    },
                    tiles::Pos::BottomLeft,
                    &vec,
                );
                vec = get_capture(
                    piece,
                    board,
                    if auxn > 0 && auxn <= 32 {
                        Some(auxn)
                    } else {
                        None
                    },
                    tiles::Pos::BottomRight,
                    &vec,
                );
            } else {
                vec = get_capture(
                    piece,
                    board,
                    if auxn > 0 && auxn <= 32 {
                        Some(auxn)
                    } else {
                        None
                    },
                    tiles::Pos::TopRight,
                    &vec,
                );
                vec = get_capture(
                    piece,
                    board,
                    if auxn > 0 && auxn <= 32 {
                        Some(auxn)
                    } else {
                        None
                    },
                    tiles::Pos::TopLeft,
                    &vec,
                );
            }
        }
    }
    return vec;
}

pub fn rec_contains(n: u8, ivec: Option<&Vec<Box<Capture>>>) -> bool {
    if let Some(vec) = ivec {
        for i in vec {
            if i.ndest == n {
                return true;
            }
            if i.next.len() > 0 {
                return rec_contains(n, Some(&i.next));
            }
        }
    }
    return false;
}

pub fn get_max_capture_depth(
    abs_level: Option<&mut u8>,
    this_level: u8,
    vec: &Vec<Box<Capture>>,
) -> u8 {
    if let Some(lev) = abs_level {
        if this_level > *lev {
            *lev = this_level;
        }
        for i in vec {
            if i.next.len() > 0 {
                *lev = get_max_capture_depth(Some(lev), this_level + 1, &i.next);
            }
        }
        return *lev;
    } else {
        let mut lev: u8 = this_level;
        return get_max_capture_depth(Some(&mut lev), this_level, vec);
    }
}

fn rec_mark_as_invalid(
    n: u8,
    vec: &Vec<Box<Capture>>,
    board: &mut board::Board,
    level: u8,
    max_depth: u8,
) -> bool {
    for i in vec {
        if level == max_depth {
            if i.ndest == n {
                let index: usize = board.tiles[(i.ncapture - 1) as usize] as usize;
                board.pieces[index - 1].valid = false;
                board.tiles[(i.ncapture - 1) as usize] = 0;
                return true;
            }
        }
        if i.next.len() > 0 {
            if rec_mark_as_invalid(n, &i.next, board, level + 1, max_depth) {
                let index: usize = board.tiles[(i.ncapture - 1) as usize] as usize;
                board.pieces[index - 1].valid = false;
                board.tiles[(i.ncapture - 1) as usize] = 0;
                return true;
            }
        }
    }
    return false;
}

pub fn eat(nfrom: u8, nto: u8, board: &mut board::Board, vec: &Vec<Box<Capture>>) -> bool {
    if rec_mark_as_invalid(nto, vec, board, 0, get_max_capture_depth(None, 0, vec)) {
        piece::move_to(nfrom, nto, board);
        return true;
    }
    return false;
}
