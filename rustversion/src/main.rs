mod board;
mod cell;

fn main() {
    let rows = 10;
    let cols = 10;
    // let iterations = 100;
    let mut game_board: board::Board= board::get_new_board(rows, cols);
    // println!("{:?}", gameBoard);
    for _ in 0..100 {
        game_board.show();
        println!();
        // pause for half a second 
        std::thread::sleep(std::time::Duration::from_millis(500));
        game_board.update();
    }
}        