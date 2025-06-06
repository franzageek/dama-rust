use raylib::{prelude::*};

use crate::{board, capture, coord, piece};

pub const WINDOW_SIZE: u16 = 640;
pub const TILE_SIZE: u16 = WINDOW_SIZE / 8;

pub fn draw_capture_hints(d: &mut RaylibDrawHandle, vec: &Vec<capture::Capture>) -> bool {
    for i in vec {
        let (x, y) = coord::xy_from_n(i.ndest);
        d.draw_circle(
            (x as u16 * TILE_SIZE  + TILE_SIZE  / 2) as i32, 
            (y as u16 * TILE_SIZE + TILE_SIZE  / 2) as i32, 
            ((TILE_SIZE - 10) / 2) as f32,
            color::rcolor(0xff, 0xff, 0, 0x5f)
        );
    }
    return vec.len() > 0;
}

pub fn draw_hints(d: &mut RaylibDrawHandle, piece: Option<&mut piece::Piece>, tiles: &Vec<u8>, capture_available: bool) {
    if !capture_available {
        if let Some(piece2) = piece {
            let vec: Vec<u8> = piece::possible_moves(piece2, tiles);
            for i in vec {
                let (x, y) = coord::xy_from_n(i);
                d.draw_circle(
                    (x as u16 * TILE_SIZE  + TILE_SIZE  / 2) as i32, 
                    (y as u16 * TILE_SIZE + TILE_SIZE  / 2) as i32, 
                    ((TILE_SIZE - 10) / 2) as f32,
                    color::rcolor(0xff, 0xff, 0, 0x5f)
                );
            }
        }
    }
}

pub fn draw_board(d: &mut RaylibDrawHandle) {
    for row in 0..8 {
        for col in 0..8 {
            let x = col * TILE_SIZE;
            let y = row * TILE_SIZE;
            let color = if (row + col) % 2 == 0 {
                raylib::color::rcolor(0x7b,0x7b,0x7b, 255)
            } else {
                raylib::color::rcolor(0xdf, 0xdf, 0xdf, 0xff)
            };
            d.draw_rectangle(x as i32, y as i32, TILE_SIZE as i32, TILE_SIZE as i32, color);
        }
    }
}

pub fn draw_pieces(d:& mut RaylibDrawHandle, board: &mut board::Board) {
    for i in 0u8..32u8 {
        if board.tiles[i as usize] > 0 && board.tiles[i as usize] < 32 {
            let (x, y) = coord::xy_from_n(i+1);
            let piece: &piece::Piece = piece::from_n(i+1, board).expect("error: no valid piece found");
            d.draw_circle(
                (x as u16 * TILE_SIZE  + TILE_SIZE  / 2) as i32, 
                (y as u16 * TILE_SIZE + TILE_SIZE  / 2) as i32, 
                ((TILE_SIZE - 10) / 2) as f32, 
            if piece.player {
                    Color::WHITE
                } else {
                    Color::BLACK
                }
            );
            if piece.king {
                d.draw_circle(
                    (x as u16 * TILE_SIZE + TILE_SIZE / 2) as i32, 
                    (y as u16 * TILE_SIZE + TILE_SIZE / 2) as i32, 
                    ((TILE_SIZE - 30) / 2) as f32, 
                    raylib::color::rcolor(0x7b,0x7b,0x7b, 255)

                );
                d.draw_circle(
                    (x as u16 * TILE_SIZE + TILE_SIZE / 2) as i32, 
                    (y as u16 * TILE_SIZE + TILE_SIZE / 2) as i32, 
                    ((TILE_SIZE - 50) / 2) as f32, 
                    if piece.player {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    }
                );
            }
        }
    }
}

