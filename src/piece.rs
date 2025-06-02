#[derive(Clone)]
pub struct Piece {
    n: u8,
    player: bool,
    king: bool,
    valid: bool
}

impl Piece {
    pub fn init() -> Vec<Piece> {
        print!("initializing pieces...");
        let mut vec: Vec<Piece> = vec![Piece { n: 0, player: false, king: false, valid: false }; 24];
        for i in 0u8..24u8 {
            vec.push(
                Piece { 
                    n: i + if i > 11{
                        9
                    } else {
                        1
                    }, 
                    player: if i > 11{
                        true
                    } else {
                        false
                    }, 
                    king: false, 
                    valid: true 
                }
            );
        }
        print!("done\n");
        return vec;
    }
}