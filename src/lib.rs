use wasm_bindgen::prelude::*;

extern crate js_sys;
extern crate web_sys;

mod life_106_codec;
mod rle_codec;
mod render;

pub use rle_codec::*;
pub use life_106_codec::*;
pub use render::*;

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

fn coordinates_to_idx(
    x: u32, y: u32,
    w: u32, h: u32,
) -> Option<usize> {
    if x >= w || y >= h {
        return None;
    }
    Some((y*w + x) as usize)
}

fn idx_to_coordinates(
    idx: usize,
    w: u32, h: u32,
) -> Option<(u32, u32)> {
    if idx as u32 >= w*h {
        return None;
    }
    Some(((idx as u32)%w, (idx as u32)/w))
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    /// 
    /// Returns a slice of Cell values
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    fn get_index(&self, col: i32, row: i32) -> usize {
        let col = ((col%self.width  + self.width )%self.width) as u32;
        let row = ((row%self.height + self.height)%self.height) as u32;
        coordinates_to_idx(col, row, self.width as u32, self.height as u32)
            .unwrap()
    }

    fn get_coordinates(&self, index: usize) -> (i32, i32) {
        let index = index%self.cells.len();
        idx_to_coordinates(index, self.width as u32, self.height as u32)
            .map(|(x, y)| (x as i32, y as i32))
            .unwrap()
    }

    fn live_neighbour_count(&self, col: i32, row: i32) -> u8 {
        let mut count = 0;

        for delta_row in [-1, 0, 1].iter().cloned() {
            for delta_col in [-1, 0, 1].iter().cloned() {
                if delta_row != 0 || delta_col != 0 {
                    let idx = self.get_index(col + delta_col, row + delta_row);
                    count += self.cells[idx] as u8;
                }
            }
        }

        count
    }
}

fn js_array_to_coordinate_tuple(
    array: &js_sys::Array,
) -> Result<(i32, i32), JsError> {
    (0..=1)
        .map(|i| array.get(i).as_f64().ok_or(JsError::new("Invalid type")))
        .collect::<Result<Vec<_>, JsError>>()
        .map(|v| (v[0] as i32, v[1] as i32))
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let width = 1.max(width as i32);
        let height = 1.max(height as i32);
        let cells = vec![Cell::Dead; (width*height) as usize];

        Universe {
            width,
            height,
            cells,
        }
    }

    /// Set all cells to the dead state.
    pub fn clear(&mut self) {
        self.cells = (0..self.width*self.height).map(|_| Cell::Dead).collect();
    }

    /// Randomly set cells to be alive or dead.
    pub fn randomize(&mut self) {
        self.cells = (0..self.width*self.height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    /// Set the width of the universe.
    /// 
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, new_width: i32) {
        let new_width = 1.max(new_width) as u32;

        let mut cells = vec![Cell::Dead; (new_width*self.height as u32) as usize];

        for y in 0..(self.height as u32) {
            for x in 0..new_width.min(self.width as u32) {
                let new_idx = coordinates_to_idx(x, y, new_width as u32, self.height as u32).unwrap();
                let idx = coordinates_to_idx(x, y, self.width as u32, self.height as u32).unwrap();
                cells[new_idx] = self.cells[idx];
            }
        }

        self.cells = cells;
        self.width = new_width as i32;
    }

    /// Set the height of the universe.
    /// 
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, new_height: i32) {
        let new_height = 1.max(new_height) as u32;

        let mut cells = vec![Cell::Dead; (self.width as u32*new_height) as usize];

        for y in 0..new_height.min(self.height as u32) {
            for x in 0..(self.width as u32) {
                let new_idx = coordinates_to_idx(x, y, self.width as u32, new_height as u32).unwrap();
                let idx = coordinates_to_idx(x, y, self.width as u32, self.height as u32).unwrap();
                cells[new_idx] = self.cells[idx];
            }
        }

        self.cells = cells;
        self.height = new_height as i32;
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn translate(&mut self, dx: i32, dy: i32) {
        let mut new_cells = vec![Cell::Dead; (self.width*self.height) as usize];
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, &cell)| cell == Cell::Alive)
            .for_each(|(idx, &cell)| {
                let (x, y) = self.get_coordinates(idx);
                let new_idx = self.get_index(x + dx, y + dy);
                new_cells[new_idx] = cell;
            });
        self.cells = new_cells;
    }

    /// Get the state of a cell in the universe.
    pub fn get_cell(&self, col: i32, row: i32) -> Cell {
        let idx = self.get_index(col, row);
        self.cells[idx]
    }

    /// Set the state of a cell in the universe.
    pub fn set_cell(&mut self, col: i32, row: i32, state: Cell) {
        let idx = self.get_index(col, row);
        self.cells[idx] = state;
    }

    /// Set the state of a list of cells in the universe.
    /// 
    /// Expects an array of arrays of the form [[row, col], [row, col], ...]
    pub fn set_cells(
        &mut self,
        cells: js_sys::Array,
        state: Cell,
    ) -> Result<(), JsError> {
        cells
            .iter()
            .map(|value| {
                if value.is_array() {
                    let cell = value.unchecked_into::<js_sys::Array>();
                    let (col, row) = js_array_to_coordinate_tuple(&cell)?;
                    self.set_cell(col, row, state);
                    Ok(())
                } else { Err(JsError::new("Invalid type")) }
            })
            .collect::<Result<(), JsError>>()
    }

    /// Set the state of a cell in the universe.
    pub fn toggle_cell(&mut self, col: i32, row: i32) {
        let idx = self.get_index(col, row);
        self.cells[idx].toggle();
    }

    /// Toggle the state of a cell in the universe.
    /// 
    /// Expects an array of arrays of the form [[row, col], [row, col], ...]
    pub fn toggle_cells(&mut self, cells: js_sys::Array)
        -> Result<(), JsError> {
        cells
            .iter()
            .map(|value| {
                let cell = value.unchecked_into::<js_sys::Array>();
                let (col, row) = js_array_to_coordinate_tuple(&cell)?;
                self.toggle_cell(col, row);
                Ok(())
            })
            .collect::<Result<(), JsError>>()
    }

    pub fn tick(&mut self) {
        let mut cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(col, row);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(col, row);

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
}
