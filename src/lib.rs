mod utils;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/**
 * Represents a cell in the universe.
 */
#[wasm_bindgen]
#[repr(u8)] // Ensure each cell is represented as a single byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

/**
 * Represents the game's universe
 */
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /**
     * Get index of cell in the universe
     */
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /**
     * Calculate the number of live neighbors
     */
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {

                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    /**
     * Initialize a 64 x 64 universe
     */
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
                    .map(|i| {
                        if i % 2 == 0 || i % 7 == 0 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    })
                    .collect();
        
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    /**
     * Updates universe
     */
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    //  Underpopulation. Any cell with less than two neighbors dies
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    //  Cell has 2/3 live neighbors lives on to the next generation
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    //  Overpopulation. Any cell with more than 3 live neighbors dies
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    //  Repopulation, Cell has 3 live neighbors it becomes alive
                    (Cell::Dead, 3) => Cell::Alive,
                    //  All other cells remain in the same state
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '???' } else { '???' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}


