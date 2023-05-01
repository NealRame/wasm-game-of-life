use wasm_bindgen::prelude::*;

use crate::*;

/******************************************************************************
 * Encoder
 *****************************************************************************/

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

/******************************************************************************
 * Decoder
 *****************************************************************************/

pub enum Life106DecoderError {
    InvalidType,
    InvalidFormat,
}

impl std::fmt::Display for Life106DecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Life106DecoderError::InvalidType => write!(f, "Invalid type"),
            Life106DecoderError::InvalidFormat => write!(f, "Invalid format"),
        }
    }
}

impl From<Life106DecoderError> for JsValue {
    fn from(err: Life106DecoderError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

fn check_if(
    cond: bool,
    err: Life106DecoderError,
) -> Result<(), Life106DecoderError> {
    if cond
    { Ok(()) } else { Err(err) }
}

#[wasm_bindgen]
impl Universe {
pub fn from_life_106(value: JsValue) -> Result<Universe, Life106DecoderError> {
    check_if(value.is_string(), Life106DecoderError::InvalidType)?;

    let life_106_string = value.as_string().unwrap();

    let mut cells = Vec::new();
    for line in life_106_string.lines().filter(|line| !line.starts_with("#")) {
        let parts = line
            .split_whitespace()
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();

        check_if(parts.len() == 2, Life106DecoderError::InvalidFormat)?;

        let x = parts[0].parse::<i32>().or(Err(Life106DecoderError::InvalidFormat))?;
        let y = parts[1].parse::<i32>().or(Err(Life106DecoderError::InvalidFormat))?;

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
