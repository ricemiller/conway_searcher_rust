extern crate rand;
use rand::Rng;

const SIZE_GRID: usize = 8;

pub struct Conway {
    grid: [[bool; SIZE_GRID]; SIZE_GRID],
    alive: bool,
}

impl Conway {
    pub fn new() -> Conway {
        return Conway {grid: [[false; SIZE_GRID]; SIZE_GRID], alive: false}
    }

    pub fn gen_rnd_conway(&mut self) {
        self.alive = false;

        for i in 0..SIZE_GRID {
            for j in 0..SIZE_GRID {
                if rand::thread_rng().gen_range(0, 100) > 70 {
                    self.grid[i][j] = true;
                    self.alive = true;
                }
            }
        }
    }

    pub fn print(&mut self) {
        for i in 0..SIZE_GRID {
            for j in 0..SIZE_GRID {
                if self.grid[i][j] {
                    print!(" # ");
                } else {
                    print!(" . ");
                }
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn evolve(&mut self) {
        let mut new_grid = [[false; SIZE_GRID]; SIZE_GRID];
        let mut up: usize;
        let mut down: usize;
        let mut left: usize;
        let mut right: usize;

        self.alive = false;

        for i in 0..SIZE_GRID {
            for j in 0..SIZE_GRID {
                match i {
                    0 => {
                        up = SIZE_GRID-1;
                        down = i+1;
                    }
                    a if a == SIZE_GRID-1 => {
                        up = i-1;
                        down = 0;
                    }
                    _ => {
                        up = i-1;
                        down = i+1;
                    }
                }

                match j {
                    0 => {
                        left = SIZE_GRID-1;
                        right = j+1;
                    }
                    a if a == SIZE_GRID-1 => {
                        left = j-1;
                        right = 0;
                    }
                    _ => {
                        left = j-1;
                        right = j+1;
                    }
                }

                let mut neighbours: u8 = 0;

                if self.grid[up][j] {
                    neighbours += 1;
                }
                if self.grid[down][j] {
                    neighbours += 1;
                }
                if self.grid[i][left] {
                    neighbours += 1;
                }
                if self.grid[i][right] {
                    neighbours += 1;
                }

                match neighbours {
                    0 | 1 | 4 => new_grid[i][j] = false,
                    2 => new_grid[i][j] = self.grid[i][j],
                    3 => new_grid[i][j] = true,
                    _ => {},
                }

                if !self.alive {
                    self.alive = self.alive || new_grid[i][j];
                }
            }
        }
        self.grid = new_grid;
    }

    pub fn is_alive(&mut self) -> bool {
        return self.alive;
    }


}
