use crate::piece::*;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Board {
    pub pieces: Vec<Piece>,
    pub tiles: Vec<u8>,
    pub state: bool
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