mod piece;
mod board;

fn main() {
    println!("dama rust rewrite, v0.1");
    let board: board::Board = board::Board::init();
    println!("Thanks for playing dama ;)");
}
