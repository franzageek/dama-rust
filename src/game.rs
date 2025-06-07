use raylib::{prelude::RaylibDraw, *};

use crate::{board, capture, coord, piece, tiles, ui};

fn can_become_king(n: u8, board: &mut board::Board) {
    let piece: &mut piece::Piece = &mut board.pieces[(board.tiles[(n - 1) as usize] - 1) as usize];
    if !piece.king {
        let (_, y) = coord::xy_from_n(piece.n);
        if (piece.player && y == 7) || (!piece.player && y == 0) {
            piece.king = true;
        }
    }
}

pub fn main_loop((mut handle, thread): (RaylibHandle, RaylibThread), board: &mut board::Board) {
    let mut nfrom: u8 = 0;
    while !handle.window_should_close() {
        if handle.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = handle.get_mouse_position();

            let col = (mouse_pos.x as i32) / ui::TILE_SIZE as i32;
            let row = (mouse_pos.y as i32) / ui::TILE_SIZE as i32;

            if row >= 0 && row < 8 && col >= 0 && col < 8 {
                println!("click[x={}, y={}]", col, row);
                if (row % 2 == 0) == (col % 2 == 0) {
                    let thisn: u8 = coord::n_from_xy((col as u8, row as u8));
                    println!("thisn:{thisn}");
                    if nfrom == 0 {
                        if board.tiles[(thisn - 1) as usize] != 0 {
                            if let Some(piece) = piece::from_n(thisn, board) {
                                if piece.player == board.state {
                                    nfrom = thisn;
                                    println!("selected piece at [x={col}|y={row}|n={thisn}]");
                                } else {
                                    println!("[E] it's player {} turn!", board.state);
                                }
                            } else {
                                unreachable!("error: could not retrieve piece from board::tiles");
                            }
                        } else {
                            println!("[E] tile at [x={col}|y={row}|n={thisn}] is free");
                        }
                    } else {
                        if nfrom != thisn {
                            println!("mark tile at [x={col}|y={row}|n={thisn}] as destination");
                            if let Some(piece) = piece::from_n(nfrom, &mut board.clone()) {
                                // `piece` is a clone of a piece
                                let captures: Vec<Box<capture::Capture>> = capture::get_possible(
                                    Some(piece),
                                    &mut board.clone(),
                                    0,
                                    tiles::Pos::None,
                                );
                                if captures.len() > 0 {
                                    if capture::rec_contains(thisn, Some(&captures))
                                        && capture::eat(nfrom, thisn, board, &captures)
                                    {
                                        println!("piece was moved successfully");
                                        can_become_king(thisn, board);
                                        board.state = !board.state;
                                    } else {
                                        println!("[E] cannot move piece: illegal move");
                                    }
                                } else {
                                    let vec: Vec<u8> =
                                        piece::possible_moves(piece, &board.tiles.clone());
                                    println!("possible moves: {:?}", vec);
                                    if vec.contains(&thisn) {
                                        piece::move_to(piece.n, thisn, board); // used here for `piece.n`
                                        println!("piece was moved successfully");
                                        can_become_king(thisn, board);
                                        board.state = !board.state;
                                    } else {
                                        println!("[E] cannot move piece: illegal move");
                                    }
                                }
                            } else {
                                unreachable!("error: could not retrieve piece from board::tiles");
                            }
                        } else {
                            println!("[E] cannot move piece: destination is equal to source");
                        }
                        nfrom = 0;
                    }
                } else {
                    println!("[E] white tiles are discarded");
                    nfrom = 0;
                }
            }
        }
        let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
        rldh.clear_background(raylib::color::rcolor(0xDF, 0xDF, 0xDF, 0xFF));
        ui::draw_board(&mut rldh);
        ui::draw_pieces(&mut rldh, board);
        let vec: Vec<Box<capture::Capture>> = capture::get_possible(
            piece::from_n(nfrom, &mut board.clone()),
            board,
            0,
            tiles::Pos::None,
        );
        let capture_available: bool = ui::draw_capture_hints(
            &mut rldh,
            &vec,
            0,
            capture::get_max_capture_depth(None, 0, &vec),
            None,
        );
        ui::draw_hints(
            &mut rldh,
            piece::from_n(nfrom, &mut board.clone()),
            &board.tiles,
            capture_available,
        );
        rldh.draw_text("test", 12, 10, 20, color::rcolor(0xFF, 0x7f, 0, 0xFF));
    }
}
