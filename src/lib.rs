mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;

extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
}

impl Universe {
    /// Set the width of the universe.
    /// 
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: i32) {
        self.width = 1.max(width) ;
        self.cells = (0..self.width*self.height).map(|_| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    /// 
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: i32) {
        self.height = 1.max(height);
        self.cells = (0..self.width*self.height).map(|_| Cell::Dead).collect();
    }

    /// Get the dead and alive values of the entire universe.
    /// 
    /// Returns a slice of Cell values
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in the universe given a list of coordinates.
    /// 
    /// Coordinates are given as a list of (row, column) tuples.
    pub fn set_cells(&mut self, cells: &[(i32, i32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    fn get_index(&self, row: i32, col: i32) -> usize {
        let row = (row%self.height + self.height)%self.height;
        let col = (col%self.width  + self.width )%self.width;
        (row*self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: i32, col: i32) -> u8 {
        let mut count = 0;

        for delta_row in [-1, 0, 1].iter().cloned() {
            for delta_col in [-1, 0, 1].iter().cloned() {
                if delta_row != 0 || delta_col != 0 {
                    let idx = self.get_index(row + delta_row, col + delta_col);
                    count += self.cells[idx] as u8;
                }
            }
        }

        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let width = 64;
        let height = 64;
        let cells =
            (0..width*height)
                .map(|_| {
                    if js_sys::Math::random() < 0.5 {
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

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: i32, col: i32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    pub fn generate_glider(&mut self, row: i32, col: i32) {
        self.set_cells(&[
            (row + 1, col - 1),
            (row - 1, col),
            (row + 1, col),
            (row, col + 1),
            (row + 1, col + 1),
        ]);
    }

    pub fn generate_pulsar(&mut self, row: i32, col: i32) {
        self.set_cells(&[
            (row - 2, col - 1), (row - 3, col - 1), (row - 4, col - 1),
            (row - 1, col - 2), (row - 6, col - 2),
            (row - 1, col - 3), (row - 6, col - 3),
            (row - 1, col - 4), (row - 6, col - 4),
            (row - 2, col - 6), (row - 3, col - 6), (row - 4, col - 6),
            (row + 2, col - 1), (row + 3, col - 1), (row + 4, col - 1),
            (row + 1, col - 2), (row + 6, col - 2),
            (row + 1, col - 3), (row + 6, col - 3),
            (row + 1, col - 4), (row + 6, col - 4),
            (row + 2, col - 6), (row + 3, col - 6), (row + 4, col - 6),
            (row - 2, col + 1), (row - 3, col + 1), (row - 4, col + 1),
            (row - 1, col + 2), (row - 6, col + 2),
            (row - 1, col + 3), (row - 6, col + 3),
            (row - 1, col + 4), (row - 6, col + 4),
            (row - 2, col + 6), (row - 3, col + 6), (row - 4, col + 6),
            (row + 2, col + 1), (row + 3, col + 1), (row + 4, col + 1),
            (row + 1, col + 2), (row + 6, col + 2),
            (row + 1, col + 3), (row + 6, col + 3),
            (row + 1, col + 4), (row + 6, col + 4),
            (row + 2, col + 6), (row + 3, col + 6), (row + 4, col + 6),
        ]);
    }

    pub fn clear(&mut self) {
        self.cells = (0..self.width*self.height).map(|_| Cell::Dead).collect();
    }

    pub fn tick(&mut self) {
        let mut cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                let next_cell = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                cells[idx] = next_cell;
            }
        }

        self.cells = cells;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                write!(f, "{}", if cell == Cell::Dead {
                    '◻'
                } else {
                    '◼'
                })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}