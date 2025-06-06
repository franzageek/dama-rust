use crate::{board, piece, tiles::{self, TileState}};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Capture {
    pub ndest: u8,
    pub ncapture: u8,
    pub next: Vec<Capture>
}

fn get_capture(piece: &mut piece::Piece, board: &mut board::Board, auxn: Option<u8>, pos: tiles::Pos, ivec: Vec<Capture>) -> Vec<Capture> {
    let mut vec: Vec<Capture> = Vec::with_capacity(0);
    if ivec.len() > 0 {
        vec = ivec.clone();
    }
    match tiles::get_next(
        pos, 
        match auxn {
            Some(aux_n) => { aux_n }
            _ => { piece.n }
        }, 
        &board.tiles
    ) {
        tiles::TileState::Busy(n0) => {
            if let Some(piece0) = piece::from_n(n0, board) {
                if piece0.king == piece.king || (piece.king && !piece0.king) {
                    match tiles::get_next(pos, n0, &board.tiles) {
                        TileState::Free(n1) => {
                            vec.push(
                                Capture { 
                                    ndest: n1, 
                                    ncapture: n0, 
                                    next: get_possible(Some(piece), board, n1, pos)
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
    return vec.to_vec();
}

pub fn get_possible(in_piece: Option<&mut piece::Piece>, board: &mut board::Board, auxn: u8, last_move: tiles::Pos) -> Vec<Capture> {
    let mut vec: Vec<Capture> = Vec::with_capacity(0);
    if let Some(piece) = in_piece {
        if piece.king {
            for i in 1u8..=4u8 {
                if last_move as u8 > 0 {
                    if i != ((last_move as u8 + 1) % 4) + 1 {
                        vec = get_capture(piece, board, if auxn > 1 && auxn < 32 {
                            Some(auxn)
                        } else {
                            None
                        }, 
                        tiles::Pos::from(i), vec); //fix cast
                    }
                }
                else {
                    vec = get_capture(piece, board, if auxn > 1 && auxn < 32 {
                        Some(auxn)
                    } else {
                        None
                    }, 
                    tiles::Pos::from(i), vec);
                }
            }
        } else {
            if piece.player {
                vec = get_capture(piece, board, if auxn > 1 && auxn < 32 {
                    Some(auxn)
                } else {
                    None
                }, 
                tiles::Pos::BottomLeft, vec);
                vec = get_capture(piece, board, if auxn > 1 && auxn < 32 {
                    Some(auxn)
                } else {
                    None
                }, 
                tiles::Pos::BottomRight, vec);
            } else {
                vec = get_capture(piece, board, if auxn > 1 && auxn < 32 {
                    Some(auxn)
                } else {
                    None
                }, 
                tiles::Pos::TopRight, vec);
                vec = get_capture(piece, board, if auxn > 1 && auxn < 32 {
                    Some(auxn)
                } else {
                    None
                }, 
                tiles::Pos::TopLeft, vec);
            }
        }
    } 
    return vec;
}