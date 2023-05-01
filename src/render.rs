use wasm_bindgen::prelude::*;

use crate::*;

#[wasm_bindgen]
impl Universe {
pub fn render_to_string(&self) -> String {
    let mut str = String::new();
    for line in self.cells.as_slice().chunks(self.width as usize) {
        for &cell in line {
            match cell {
                Cell::Dead => str.push('◻'),
                Cell::Alive => str.push('◼'),
            }
        }
        str.push('\n');
    }
    str
}

/// Render the universe to a canvas element.
pub fn render_to_context(
    &self,
    context: web_sys::CanvasRenderingContext2d,
    theme: JsValue,
) -> Result<(), String> {
    let cell_size = match js_sys::Reflect::get(&theme, &"cellSize".into()) {
        Ok(value) => value.as_f64().ok_or("cellSize should be a number")?,
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

        let (col, row) = self.get_coordinates(idx);
        context.fill_rect(
            (col as f64)*(cell_size + 1.0) + 1.0,
            (row as f64)*(cell_size + 1.0) + 1.0,
            cell_size,
            cell_size,
        );
    }
    context.stroke();

    Ok(())
}}
