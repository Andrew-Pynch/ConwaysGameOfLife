use core::fmt;

#[derive(Debug)]
pub struct Board {
    rows: i32,
    cols: i32,
}

pub fn get_new_board(rows: i32, cols: i32) -> Board {
    return Board {
        rows: rows,
        cols: cols,
    }
}


// pub fmt::Debug for Board {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Board {{ rows: {}, cols: {} }}", self.rows, self.cols)
//     }
// }
