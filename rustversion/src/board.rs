use crate::cell::{self, Cell}; 
use rand::Rng;



#[derive(Debug)]
pub struct Board {
    rows: i32,
    cols: i32,
    grid: Vec<Vec<Cell>>,
}

pub fn generate_board(instantiated_board: Board) -> Board {
    let mut new_board = instantiated_board;
    for row in 0..new_board.rows {
        for col in 0..new_board.cols {
            let chance_number = rand::thread_rng().gen_range(0 as i32, 3 as i32);
            
            let mut new_cell: Cell = cell::get_new_cell(); 

            // 33 chance to spawn
            if chance_number == 1 {
                new_cell.set_alive();
            } else {
                new_cell.set_dead();
            }

            new_board.grid[row as usize][col as usize] = new_cell
        }
    }

    return new_board 
}

pub fn get_new_board(rows: i32, cols: i32) -> Board {
    let fake_cell: Cell = cell::get_new_cell();
    let instantiated_board= Board {
        rows: rows,
        cols: cols,
        grid: vec![vec![fake_cell; cols as usize]; rows as usize],
    };

    let generated_board = generate_board(instantiated_board);
    
    return generated_board;
}

impl Board {
    pub fn show(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
               self.grid[row as usize][col as usize].show();
            }
            println!();
        }
    }
    
    pub fn update(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut cell = self.grid[row as usize][col as usize];
                let (alive_neighbors, mut _neighbors) = self.get_cell_neighbor_info(cell);
                
                // conways rules!
                if cell.is_alive() {
                   if alive_neighbors.len() < 2 {
                    cell.set_dead();
                   }  else if alive_neighbors.len() == 2 || alive_neighbors.len() == 3 {
                    cell.set_alive();
                   } else if alive_neighbors.len() > 3 {
                    cell.set_dead();
                   }
                } else if cell.is_dead() {
                    if alive_neighbors.len() == 3 {
                        cell.set_alive();
                    }
                }

                // update the cell at the current position on the board
                self.grid[row as usize][col as usize] = cell;
            }
        }
    }

    pub fn get_cell_neighbor_info(&self, cell: Cell) -> (Vec<Cell>, Vec<Cell>) {
        let mut alive_neighbors: Vec<Cell> = Vec::new();
        let mut dead_neighbors: Vec<Cell> = Vec::new();

        let (cell_row, cell_col) = cell.get_coordinates();

        for row in 0..3 {
            for col in 0..3 {
                    let mut neighbor: Cell = cell::get_new_cell();
                    neighbor =  self.grid[cell_row as usize -1 + row][cell_col as usize - 1 + col];
                    if neighbor.get_coordinates() != cell.get_coordinates() {
                        if neighbor.is_alive() {
                            alive_neighbors.push(neighbor);
                        } else {
                            dead_neighbors.push(neighbor);
                        }
                     }
            }
        }
        
        return (alive_neighbors, dead_neighbors);
    }
}