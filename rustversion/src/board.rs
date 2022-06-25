use rand::Rng;


#[derive(Debug)]
pub struct Board {
    rows: i32,
    cols: i32,
    grid: Vec<Vec<char>>,
}

pub fn generate_board(instantiated_board: Board) -> Board {
    let mut new_board = instantiated_board;
    for row in 0..new_board.rows {
        for col in 0..new_board.cols {
            let chance_number = rand::thread_rng().gen_range(0, 3);
            if chance_number == 1 {
                new_board.grid[row as usize][col as usize] = 'X';
            } else {
                new_board.grid[row as usize][col as usize] = 'O';
            }
        }
    }

    return new_board 
}

pub fn get_new_board(rows: i32, cols: i32) -> Board {
    let instantiated_board= Board {
        rows: rows,
        cols: cols,
        grid: vec![vec![' '; cols as usize]; rows as usize],
    };

    let generated_board = generate_board(instantiated_board);
    
    return generated_board;
}

impl Board {
    pub fn print_board(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!(" {} ", self.grid[row as usize][col as usize]);
            }
            println!();
        }
    }

    pub fn update(&self) {
        
    }
}

// pub impl Board {
//     pub fn show() {
// println!("{:?}", self.grid);
//     }
    
// }


// pub fmt::Debug for Board {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Board {{ rows: {}, cols: {} }}", self.rows, self.cols)
//     }
// }
