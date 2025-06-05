use raylib::{prelude::RaylibDraw, *};

use crate::{board, ui};

pub fn main_loop((mut handle, thread): (RaylibHandle, RaylibThread), board: &mut board::Board) {
    while !handle.window_should_close() {
        let mut rldh: core::drawing::RaylibDrawHandle =  handle.begin_drawing(&thread);
        rldh.clear_background(color::rcolor(0xFF, 0xFF, 0xFF, 0xFF));
        ui::draw_board(&mut rldh);
        ui::draw_pieces(&mut rldh, board);
        rldh.draw_text("test", 12, 10, 20, color::rcolor(0xFF, 0x7f, 0, 0xFF));
    }
}