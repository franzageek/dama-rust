use raylib::{prelude::RaylibDraw, *};

use crate::{board, coord, piece, ui};

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
                        if board.tiles[(thisn-1) as usize] != 0 {
                            if let Some(piece) = piece::from_n(thisn, board) {
                                if piece.player == board.state {
                                    nfrom = thisn;
                                    println!("selected piece at [x={col}|y={row}|n={thisn}]");
                                } else {
                                    println!("[E] it's player {} turn!", board.state);
                                }
                            }
                        } else {
                            println!("[E] tile at [x={col}|y={row}|n={thisn}] is free");
                        }
                    } else {
                        if nfrom != thisn {
                            println!("mark tile at [x={col}|y={row}|n={thisn}] as destination");
                            if let Some(piece) = piece::from_n(nfrom, &mut board.clone()) { // `piece` is a clone of a piece
                                let vec: Vec<u8> = piece::possible_moves(piece, &board.tiles.clone());
                                println!("possible moves: {:?}", vec);
                                if vec.contains(&thisn) {
                                    piece::move_to(piece.n, thisn, board); // used here for `piece.n`
                                    println!("piece was moved successfully");
                                    if let Some(piece2) = piece::from_n(thisn, board) { // here we get the actual piece
                                        if piece2.n != thisn {
                                            println!("[E] cannot move piece: illegal move")
                                        } else {
                                            board.state = !board.state;
                                        }
                                    }
                                } else {
                                    println!("[E] cannot move piece: illegal move");
                                }
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
        let mut rldh: core::drawing::RaylibDrawHandle =  handle.begin_drawing(&thread);
        rldh.clear_background(color::rcolor(0xFF, 0xFF, 0xFF, 0xFF));ui::draw_board(&mut rldh);
        ui::draw_pieces(&mut rldh, board);
        rldh.draw_text("test", 12, 10, 20, color::rcolor(0xFF, 0x7f, 0, 0xFF));
    }
}