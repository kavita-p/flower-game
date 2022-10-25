mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flower {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    flowers: Vec<Flower>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn count_live_neighbors(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.flowers[idx] as u8;
            }
        }
        count
    }

    pub fn get_flowers(&self) -> &[Flower] {
        &self.flowers
    }

    pub fn set_flowers(&mut self, flowers: &[(u32, u32)]) {
        for (row, col) in flowers.iter().cloned() {
            let idx = self.get_index(row, col);
            self.flowers[idx] = Flower::Alive;
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let flowers = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Flower::Alive
                } else {
                    Flower::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            flowers,
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.flowers = (0..width * self.height).map(|_| Flower::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.flowers = (0..self.width * height).map(|_| Flower::Dead).collect();
    }

    pub fn tick(&mut self) {
        let mut next = self.flowers.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let flower = self.flowers[idx];
                let live_neighbors = self.count_live_neighbors(row, col);

                log!(
                    "flower[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    flower,
                    live_neighbors
                );

                let next_flower = match (flower, live_neighbors) {
                    // Rule One. A living flower with less than two living neighbors is cut off. It dies.
                    (Flower::Alive, x) if x < 2 => Flower::Dead,
                    // Rule Two. A living flower with two or three living neighbors is connected. It lives.
                    (Flower::Alive, 2) | (Flower::Alive, 3) => Flower::Alive,
                    // Rule Three. A living flower with more than three living neighbors is starved and overcrowded. It dies.
                    (Flower::Alive, x) if x > 3 => Flower::Dead,
                    // Rule Four. A dead flower with exactly three living neighbors is reborn. It springs back to life.
                    (Flower::Dead, 3) => Flower::Alive,
                    // The only play permitted in the game is the arrangement of the initial flowers.
                    (otherwise, _) => otherwise,
                };

                log!("    it becomes {:?}", next_flower);

                next[idx] = next_flower;
            }
        }

        self.flowers = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.flowers.as_slice().chunks(self.width as usize) {
            for &flower in line {
                let symbol = if flower == Flower::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
