use raylib::{prelude::RaylibDraw, *};

use crate::{board, ui};

pub fn main_loop((mut handle, thread): (RaylibHandle, RaylibThread), board: &mut board::Board) {
        while !handle.window_should_close() {
            if handle.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_LEFT_BUTTON) {
                let mouse_pos = handle.get_mouse_position();

                let col = (mouse_pos.x as i32) / ui::TILE_SIZE as i32;
                let row = (mouse_pos.y as i32) / ui::TILE_SIZE as i32;

                if row >= 0 && row < 8 && col >= 0 && col < 8 {
                    println!("click[x={}, y={}]", col, row);
                }
            }
            let mut rldh: core::drawing::RaylibDrawHandle =  handle.begin_drawing(&thread);
            rldh.clear_background(color::rcolor(0xFF, 0xFF, 0xFF, 0xFF));ui::draw_board(&mut rldh);
            ui::draw_pieces(&mut rldh, board);
            rldh.draw_text("test", 12, 10, 20, color::rcolor(0xFF, 0x7f, 0, 0xFF));
        }
}