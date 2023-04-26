use std::fmt;

use wasm_bindgen::prelude::*;

extern crate js_sys;
extern crate web_sys;

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
    /// Get the dead and alive values of the entire universe.
    /// 
    /// Returns a slice of Cell values
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    fn get_index(&self, row: i32, col: i32) -> usize {
        let row = (row%self.height + self.height)%self.height;
        let col = (col%self.width  + self.width )%self.width;
        (row*self.width + col) as usize
    }

    fn get_row_col(&self, index: usize) -> (i32, i32) {
        let row = index as i32/self.width;
        let col = index as i32%self.width;
        (row, col)
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

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Get the state of a cell in the universe.
    pub fn get_cell(&self, row: i32, col: i32) -> Cell {
        let idx = self.get_index(row, col);
        self.cells[idx]
    }

    /// Set the state of a cell in the universe.
    pub fn set_cell(&mut self, row: i32, col: i32, state: Cell) {
        let idx = self.get_index(row, col);
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
                    let (row, col) = js_array_to_coordinate_tuple(&cell)?;
                    self.set_cell(row, col, state);
                    Ok(())
                } else { Err(JsError::new("Invalid type")) }
            })
            .collect::<Result<(), JsError>>()
    }

    /// Set the state of a cell in the universe.
    pub fn toggle_cell(&mut self, row: i32, col: i32) {
        let idx = self.get_index(row, col);
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
                let (row, col) = js_array_to_coordinate_tuple(&cell)?;
                self.toggle_cell(row, col);
                Ok(())
            })
            .collect::<Result<(), JsError>>()
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

    pub fn render_str(&self) -> String {
        self.to_string()
    }

    /// Render the universe to a canvas element.
    pub fn render_to_context(
        &self,
        context: web_sys::CanvasRenderingContext2d,
        theme: JsValue,
    ) -> Result<(), JsError> {
        let cell_size = match js_sys::Reflect::get(&theme, &"cellSize".into()) {
            Ok(value) => value.as_f64().ok_or(JsError::new("cellSize should be a number"))?,
            Err(_) => 5.0,
        };

        let alive_color =
            js_sys::Reflect::get(&theme, &"aliveCell".into())
                .unwrap_or(JsValue::from_str("#000000"));

        let dead_color =
            js_sys::Reflect::get(&theme, &"deadCell".into())
                .unwrap_or(JsValue::from_str("#ffffff"));

        context.begin_path();
        for (idx, cell) in self.cells.iter().copied().enumerate() {
            if cell == Cell::Alive {
                context.set_fill_style(&alive_color);
            } else {
                context.set_fill_style(&dead_color);
            }
 
            let (row, col) = self.get_row_col(idx);
            context.fill_rect(
                (col as f64)*(cell_size + 1.0) + 1.0,
                (row as f64)*(cell_size + 1.0) + 1.0,
                cell_size,
                cell_size,
            );
        }
        context.stroke();

        Ok(())
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