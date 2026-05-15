use sudoku_core::{Difficulty, Game as CoreGame, SelectModifiers, ValidationMode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    inner: CoreGame,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: CoreGame::default(),
        }
    }

    pub fn start(&mut self, difficulty: &str, validation_mode: &str) -> Result<(), JsValue> {
        let difficulty =
            Difficulty::parse(difficulty).ok_or_else(|| JsValue::from_str("Invalid difficulty"))?;
        let validation_mode = ValidationMode::parse(validation_mode)
            .ok_or_else(|| JsValue::from_str("Invalid validation mode"))?;

        self.inner.start(difficulty, validation_mode, now_ms());
        Ok(())
    }

    pub fn new_game(&mut self) {
        self.inner.new_game();
    }

    pub fn retry(&mut self) {
        self.inner.retry(now_ms());
    }

    pub fn reset(&mut self) {
        self.inner.reset();
    }

    pub fn select_cell(&mut self, row: usize, col: usize, shift: bool, command: bool) {
        self.inner
            .select_cell(row, col, SelectModifiers { shift, command });
    }

    pub fn move_selection(&mut self, delta_row: i32, delta_col: i32) {
        self.inner.move_selection(delta_row, delta_col);
    }

    pub fn toggle_notes(&mut self) {
        self.inner.toggle_input_mode();
    }

    pub fn place_digit(&mut self, value: u8) {
        self.inner.place_digit(value);
    }

    pub fn clear_selection(&mut self) {
        self.inner.clear_selection();
    }

    pub fn submit(&mut self) {
        self.inner.submit(now_ms());
    }

    pub fn state(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner.view_state(now_ms()))
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

fn now_ms() -> u64 {
    js_sys::Date::now().max(0.0) as u64
}
