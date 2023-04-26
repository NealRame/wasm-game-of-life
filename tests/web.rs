//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate wasm_game_of_life;

use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;
use wasm_game_of_life::Cell;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_new() {
    let universe = Universe::new(3, 3);
    assert_eq!(universe.width(), 3);
    assert_eq!(universe.height(), 3);
    assert_eq!(universe.get_cells(), &[Cell::Dead; 9]);
}
