mod board;
fn main() {
    let gameBoard = board::get_new_board(3, 3);
    println!("{:?}", gameBoard);
}
