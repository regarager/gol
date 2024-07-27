extern crate termion;

use std::cmp;
use termion::terminal_size;
use rand::{RngCore};

struct Game<const N: usize> {
    rows: u16,
    cols: u16,
    state: [u64; N],
}

impl<const N: usize> Game<N> {
    fn cell(&self, row: u8, col: usize) -> u8 {
        ((self.state[col] >> row) & 1) as u8
    }

    fn neighbor_count(&self, row: u8, col: usize) -> u8 {
        let mut count = 0;

        if row > 0 {
            count += self.cell(row - 1, col);
            if col > 0 {
                count += self.cell(row - 1, col - 1);
            }
            if col < N - 1 {
                count += self.cell(row - 1, col + 1);
            }
        }
        if row < 64 - 1 {
            count += self.cell(row + 1, col);
            if col > 0 {
                count += self.cell(row + 1, col - 1);
            }
            if col < N - 1 {
                count += self.cell(row + 1, col + 1);
            }
        }

        if col > 0 {
            count += self.cell(row, col - 1);
        }

        if col < N - 1 {
            count += self.cell(row, col + 1);
        }

        count
    }

    pub fn next(&mut self) {
        let mut res: [u64; N] = [0; N];

        for i in 0..N {
            let mut col: u64 = 0;

            for j in 0..64 {
                let neighbors = self.neighbor_count(j, i);

                if self.cell(j, i) == 1 {
                    if neighbors == 2 || neighbors == 3 {
                        col = col | (1 << j)
                    }
                } else {
                    if neighbors == 3 {
                        col = col | (1 << j)
                    }
                }
            }

            res[i] = col
        }

        self.state = res
    }

    pub fn display(&self) {
        for j in 0..(cmp::min(self.rows, 64) as usize) {
            for i in 0..self.cols {
                let cell = self.state[i as usize] & (1 << j);

                if cell == 0 {
                    print!(" ");
                } else {
                    print!("\u{2B1B}");
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let (cols, rows) = terminal_size().unwrap();

    let mut state: [u64; 255] = [0; 255];

    for i in 0..255 {
        state[i] = rng.next_u64();
    }

    let mut game = Game { rows, cols, state };

    while true {
        println!("{}", termion::clear::All);
        game.display();
        game.next();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
