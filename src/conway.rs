extern crate rand;
use rand::Rng;
extern crate minifb;
use minifb::{WindowOptions, Window};

const SIZE_GRID: usize = 20;
const SIZE_SQUARE: usize = 15;
const BLACK: u32 = 0x00000000;
const BLUE: u32 = 0x000000ff;
const GREEN: u32 = 0x0000ff00;
const WIDTH: usize = SIZE_SQUARE * (SIZE_GRID + 2);

pub struct Conway {
    grid: [[bool; SIZE_GRID]; SIZE_GRID],
    window: Window,
    alive: bool,
}

impl Conway {
    pub fn new() -> Conway {
        return Conway {
            grid: [[false; SIZE_GRID]; SIZE_GRID],
            window:  Window::new("Conway",
            WIDTH,
            WIDTH,
             WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        }),
        alive: false,
    };
}

    pub fn gen_rnd_conway(&mut self) {
        self.alive = false;

        for i in 0..SIZE_GRID {
            for j in 0..SIZE_GRID {
                if rand::thread_rng().gen_range(0, 100) > 60 {
                    self.grid[i][j] = true;
                    self.alive = true;
                }
            }
        }
    }

    fn paint_square(&mut self, buffer: &mut [u32] , x: usize, y: usize, colour: u32) {
        let posx = (x + 1) * SIZE_SQUARE;
        let posy = (y + 1) * WIDTH * SIZE_SQUARE;
        for i in 1..SIZE_SQUARE-1 {
            for j in 1..SIZE_SQUARE-1 {
                buffer[posx + i + posy+ WIDTH*j] = colour;
            }
        }
    }

    pub fn print(&mut self) {

        let mut buffer= [0; WIDTH * WIDTH];
        for i in buffer.iter_mut() {
            *i = BLACK; // write something more funny here!
        }

        for i in 0..SIZE_GRID {
            for j in 0..SIZE_GRID {
                if self.grid[i][j] {
                    self.paint_square(&mut buffer, i, j, GREEN);
                } else {
                    self.paint_square(&mut buffer, i, j, BLUE);
                }
            }
        }

        self.window.update_with_buffer(&buffer).unwrap();
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
                        up = SIZE_GRID - 1;
                        down = i + 1;
                    }
                    a if a == SIZE_GRID - 1 => {
                        up = i - 1;
                        down = 0;
                    }
                    _ => {
                        up = i - 1;
                        down = i + 1;
                    }
                }

                match j {
                    0 => {
                        left = SIZE_GRID - 1;
                        right = j + 1;
                    }
                    a if a == SIZE_GRID - 1 => {
                        left = j - 1;
                        right = 0;
                    }
                    _ => {
                        left = j - 1;
                        right = j + 1;
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
                    _ => {}
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

    pub fn is_window_open (&mut self) -> bool {
        return self.window.is_open();
    }
}
