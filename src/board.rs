use crate::piece::*;
pub struct Board {
    pieces: Vec<Piece>,
    tiles: Vec<u8>,
    state: bool
}

impl Board {
    pub fn init() -> Board {
        print!("initializing board...done\n");
        return Board {
            pieces: Piece::init(),
            tiles: vec![
                1u8,2,3,4,
                5,6,7,8,
                9,10,11,12,
                0,0,0,0,
                0,0,0,0,
                13,14,15,16,
                17,18,19,20,
                21,22,23,24,
            ],
            state: true
        };
    }
}