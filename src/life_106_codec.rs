use wasm_bindgen::prelude::*;

use crate::*;

#[wasm_bindgen]
impl Universe {
pub fn to_life_106(&self) -> String {
    format!("#Life 1.06\n{}",
        self.cells.iter()
            .enumerate()
            .filter(|(_, cell)| **cell == Cell::Alive)
            .map(|(idx, _)| idx_to_coordinates(idx, self.width as u32, self.height as u32).unwrap())
            .map(|(x, y)| format!("{} {}", x, y))
            .collect::<Vec<String>>()
            .join("\n")
    )
}}

fn expect(cond: bool, message: &str) -> Result<(), JsError> {
    if cond { Ok(()) } else { Err(JsError::new(message)) }
}

#[wasm_bindgen]
impl Universe {
pub fn from_life_106(value: JsValue) -> Result<Universe, JsError> {
    expect(value.is_string(), "Expected a string")?;

    let life_106_string = value.as_string().unwrap();

    let mut cells = Vec::new();
    for line in life_106_string.lines().filter(|line| !line.starts_with("#")) {
        let parts = line
            .split_whitespace()
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();

        expect(parts.len() == 2, "Expected two numbers per line")?;

        let x = parts[0].parse::<i32>().map_err(|_| JsError::new("Expected a number"))?;
        let y = parts[1].parse::<i32>().map_err(|_| JsError::new("Expected a number"))?;

        cells.push((x, y));
    }

    let ((x_min, y_min), (x_max, y_max)) = cells
        .iter()
        .fold(
            ((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)),
            |((x_min, y_min), (x_max, y_max)), (x, y)| (
                (x_min.min(*x), y_min.min(*y)),
                (x_max.max(*x), y_max.max(*y)),
            )
        );

    let width = (x_max - x_min + 1) as u32;
    let height = (y_max - y_min + 1) as u32;

    let mut universe = Universe::new(width, height);
    for (x, y) in cells {
        universe.set_cell(
            x - x_min,
            y - y_min,
            Cell::Alive,
        );
    }

    Ok(universe)
}}
