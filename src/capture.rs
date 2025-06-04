use crate::{board, piece, tiles::{self, TileState}};

#[derive(Clone)]
#[allow(dead_code)]
pub struct Capture {
    pub ndest: u8,
    pub ncapture: u8,
    pub next: Vec<Capture>
}

fn get_capture(piece: &piece::Piece, board: &mut board::Board, auxn: Option<u8>, pos: tiles::Pos, ivec: Vec<Capture>) -> Vec<Capture> {
    let mut vec: Vec<Capture> = Vec::with_capacity(0);
    if ivec.len() > 0 {
        vec = ivec.clone();
    }
    match auxn {
        Some(aux_n) => {
            match tiles::get_next(pos, aux_n, &board.tiles) {
                tiles::TileState::Busy(n0) => {
                    if let Some(piece0) = piece::from_n(n0, board) {
                        if piece0.king == piece.king || (piece.king && !piece0.king) {
                            match tiles::get_next(pos, n0, &board.tiles) {
                                TileState::Free(n1) => {
                                    vec.push(
                                        Capture { 
                                            ndest: n1, 
                                            ncapture: n0, 
                                            next: Vec::with_capacity(0)
                                        }
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    return vec.to_vec();
}

pub fn get_possible(piece: &piece::Piece, board: &mut board::Board, auxn: u8, last_move: tiles::Pos) -> Vec<Capture> {
    let mut vec: Vec<Capture> = Vec::with_capacity(0);
    if piece.king {
        for i in 1u8..=4u8 {
            if last_move as u8 > 0 {
                if i != ((last_move as u8 + 1) % 4) + 1 {
                    vec = get_capture(piece, board, if auxn < 1 && auxn < 32 {
                        Some(auxn)
                    } else {
                        None
                    }, 
                    i, vec); //fix cast
                }
            }
            else {
                vec = get_capture(piece, board, if auxn < 1 && auxn < 32 {
                    Some(auxn)
                } else {
                    None
                }, 
                last_move, vec);
            }
        }
    } else {
        if piece.player {
            vec = get_capture(piece, board, auxn, pos, ivec)
        }
    }
    return vec;
}