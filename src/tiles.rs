use crate::coord;

#[derive(Clone, Copy)]
pub enum Pos {
    None = 0,
    BottomLeft = 1,
    BottomRight = 2,
    TopRight = 3,
    TopLeft = 4,
}

impl From<u8> for Pos {
    fn from(value: u8) -> Self {
        return match value {
            1 => Pos::BottomLeft,
            2 => Pos::BottomRight,
            3 => Pos::TopRight,
            4 => Pos::TopLeft,
            _ => Pos::None,
        };
    }
}

pub enum TileState {
    Free(u8),
    Busy(u8),
    OutOfRange,
}

pub fn get_next(pos: Pos, n: u8, tiles: &Vec<u8>) -> TileState {
    let (_, y): (u8, u8) = coord::xy_from_n(n);
    let next: u8;
    match pos {
        Pos::None => {
            next = 0;
        }
        Pos::BottomLeft => {
            next = if n != 1 && n != 9 && n != 17 && n != 25 {
                n + if y % 2 == 0 { 3 } else { 4 }
            } else {
                0
            };
        }
        Pos::BottomRight => {
            next = if n != 8 && n != 16 && n != 24 && n != 32 {
                n + if y % 2 == 0 { 4 } else { 5 }
            } else {
                0
            };
        }
        Pos::TopRight => {
            next = if y != 0 {
                if n != 8 && n != 16 && n != 24 && n != 32 {
                    n - if y % 2 == 0 { 4 } else { 3 }
                } else {
                    0
                }
            } else {
                0
            };
        }
        Pos::TopLeft => {
            next = if y != 0 {
                if n != 1 && n != 9 && n != 17 && n != 25 {
                    n - if y % 2 == 0 { 5 } else { 4 }
                } else {
                    0
                }
            } else {
                0
            };
        }
    }
    if next > 0 && next <= 32 {
        if tiles[(next - 1) as usize] == 0 {
            return TileState::Free(next);
        } else {
            return TileState::Busy(next);
        }
    } else {
        return TileState::OutOfRange;
    }
}
