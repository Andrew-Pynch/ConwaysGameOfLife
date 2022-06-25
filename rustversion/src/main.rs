mod board;
fn main() {
    let rows = 10;
    let cols = 10;
    // let iterations = 100;
    let game_board: board::Board= board::get_new_board(rows, cols);
    // println!("{:?}", gameBoard);
    game_board.print_board();
}