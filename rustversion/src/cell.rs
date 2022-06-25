#[derive(Debug, Clone, Copy)]
pub struct Cell {
    x: i32,
    y: i32,
    alive: bool,
}

pub fn get_new_cell() -> Cell {
    return Cell {
        x: 0,
        y: 0,
        alive: false,
    };
}

impl Cell {
    pub fn show(&self) {
        if self.alive {
            print!(" O ");
        } else {
            print!(" * ");
        }
    }

    pub fn set_alive(&mut self) {
        self.alive = true;
    }

    pub fn set_dead(&mut self) {
        self.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        return self.alive == true;
    }
    pub fn is_dead(&self) -> bool {
        return self.alive == false;
    }

    // pub fn set_coordinates(&mut self, x: i32, y: i32) {
    //     self.x = x;
    //     self.y = y;
    // }

    pub fn get_coordinates(&self) -> (i32, i32) {
        return (self.x, self.y);
    }
}
