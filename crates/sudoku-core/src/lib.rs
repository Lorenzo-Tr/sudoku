use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;
use ts_rs::TS;

const SIZE: usize = 9;
const BOX_SIZE: usize = 3;

pub type Puzzle = [[Option<u8>; SIZE]; SIZE];
type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    value: Option<u8>,
    notes: [bool; SIZE],
    fixed: bool,
}

impl Cell {
    fn empty() -> Self {
        Self {
            value: None,
            notes: [false; SIZE],
            fixed: false,
        }
    }

    fn fixed(value: u8) -> Self {
        Self {
            value: Some(value),
            notes: [false; SIZE],
            fixed: true,
        }
    }

    #[cfg(test)]
    fn note_values(&self) -> Vec<u8> {
        self.notes
            .iter()
            .enumerate()
            .filter_map(|(index, enabled)| enabled.then_some(index as u8 + 1))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    cells: [[Cell; SIZE]; SIZE],
}

impl Board {
    pub fn from_start(start: Puzzle) -> Self {
        let cells = start.map(|row| {
            row.map(|value| match value {
                Some(value) => Cell::fixed(value),
                None => Cell::empty(),
            })
        });

        Self { cells }
    }

    pub fn cell(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    pub fn clear_non_fixed(&mut self) {
        for row in 0..SIZE {
            for col in 0..SIZE {
                if !self.cells[row][col].fixed {
                    self.cells[row][col].value = None;
                    self.cells[row][col].notes = [false; SIZE];
                }
            }
        }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, value: Option<u8>) -> bool {
        if row >= SIZE || col >= SIZE || self.cells[row][col].fixed {
            return false;
        }

        if let Some(value) = value {
            if !(1..=9).contains(&value) {
                return false;
            }
        }

        self.cells[row][col].value = value;
        self.cells[row][col].notes = [false; SIZE];
        true
    }

    pub fn clear_value(&mut self, row: usize, col: usize) -> bool {
        self.set_cell(row, col, None)
    }

    pub fn toggle_note(&mut self, row: usize, col: usize, value: u8) -> bool {
        if row >= SIZE || col >= SIZE || !(1..=9).contains(&value) {
            return false;
        }

        if self.cells[row][col].fixed || self.cells[row][col].value.is_some() {
            return false;
        }

        let index = (value - 1) as usize;
        self.cells[row][col].notes[index] = !self.cells[row][col].notes[index];
        true
    }

    pub fn clear_notes(&mut self, row: usize, col: usize) -> bool {
        if row >= SIZE || col >= SIZE || self.cells[row][col].fixed {
            return false;
        }

        self.cells[row][col].notes = [false; SIZE];
        true
    }

    pub fn remove_related_notes(&mut self, row: usize, col: usize, value: u8) {
        if row >= SIZE || col >= SIZE || !(1..=9).contains(&value) {
            return;
        }

        let index = (value - 1) as usize;
        self.cells[row][col].notes[index] = false;

        for current_col in 0..SIZE {
            if current_col != col && !self.cells[row][current_col].fixed {
                self.cells[row][current_col].notes[index] = false;
            }
        }

        for current_row in 0..SIZE {
            if current_row != row && !self.cells[current_row][col].fixed {
                self.cells[current_row][col].notes[index] = false;
            }
        }

        let box_row = (row / BOX_SIZE) * BOX_SIZE;
        let box_col = (col / BOX_SIZE) * BOX_SIZE;
        for current_row in box_row..(box_row + BOX_SIZE) {
            for current_col in box_col..(box_col + BOX_SIZE) {
                if (current_row != row || current_col != col)
                    && !self.cells[current_row][current_col].fixed
                {
                    self.cells[current_row][current_col].notes[index] = false;
                }
            }
        }
    }

    pub fn has_conflict(&self, row: usize, col: usize) -> bool {
        let Some(value) = self.cells[row][col].value else {
            return false;
        };

        for current_col in 0..SIZE {
            if current_col != col && self.cells[row][current_col].value == Some(value) {
                return true;
            }
        }

        for current_row in 0..SIZE {
            if current_row != row && self.cells[current_row][col].value == Some(value) {
                return true;
            }
        }

        let box_row = (row / BOX_SIZE) * BOX_SIZE;
        let box_col = (col / BOX_SIZE) * BOX_SIZE;
        for current_row in box_row..(box_row + BOX_SIZE) {
            for current_col in box_col..(box_col + BOX_SIZE) {
                if (current_row != row || current_col != col)
                    && self.cells[current_row][current_col].value == Some(value)
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_complete(&self) -> bool {
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.cells[row][col].value.is_none() || self.has_conflict(row, col) {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_filled(&self) -> bool {
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.cells[row][col].value.is_none() {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../../web/src/bindings/")]
pub enum InputMode {
    Value,
    Notes,
}

impl InputMode {
    fn toggle(self) -> Self {
        match self {
            Self::Value => Self::Notes,
            Self::Notes => Self::Value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../../web/src/bindings/")]
pub enum ValidationMode {
    Live,
    Manual,
}

impl ValidationMode {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "live" | "auto" => Some(Self::Live),
            "manual" | "submit" => Some(Self::Manual),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../../web/src/bindings/")]
pub enum GameStatus {
    Playing,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../../web/src/bindings/")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "easy" => Some(Self::Easy),
            "medium" => Some(Self::Medium),
            "hard" => Some(Self::Hard),
            _ => None,
        }
    }

    fn puzzle(self) -> Puzzle {
        match self {
            Self::Easy => easy_puzzle(),
            Self::Medium => medium_puzzle(),
            Self::Hard => hard_puzzle(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SelectModifiers {
    pub shift: bool,
    pub command: bool,
}

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    current_puzzle: Puzzle,
    selected: Vec<Position>,
    input_mode: InputMode,
    validation_mode: ValidationMode,
    difficulty: Difficulty,
    show_validation_result: bool,
    started: bool,
    status: GameStatus,
    started_at_ms: Option<u64>,
    finished_elapsed_ms: Option<u64>,
}

impl Default for Game {
    fn default() -> Self {
        let puzzle = Difficulty::Easy.puzzle();

        Self {
            board: Board::from_start(puzzle),
            current_puzzle: puzzle,
            selected: vec![(0, 0)],
            input_mode: InputMode::Value,
            validation_mode: ValidationMode::Manual,
            difficulty: Difficulty::Easy,
            show_validation_result: false,
            started: false,
            status: GameStatus::Playing,
            started_at_ms: None,
            finished_elapsed_ms: None,
        }
    }
}

impl Game {
    pub fn start(&mut self, difficulty: Difficulty, validation_mode: ValidationMode, now_ms: u64) {
        self.difficulty = difficulty;
        self.validation_mode = validation_mode;
        self.start_with_puzzle(difficulty.puzzle(), now_ms);
    }

    pub fn new_game(&mut self) {
        self.reset_with_puzzle(self.difficulty.puzzle());
        self.started = false;
    }

    pub fn retry(&mut self, now_ms: u64) {
        self.start_with_puzzle(self.current_puzzle, now_ms);
    }

    pub fn reset(&mut self) {
        if self.status == GameStatus::Playing {
            self.board.clear_non_fixed();
            self.show_validation_result = false;
        }
    }

    pub fn select_cell(&mut self, row: usize, col: usize, modifiers: SelectModifiers) {
        if self.status == GameStatus::Finished || row >= SIZE || col >= SIZE {
            return;
        }

        let position = (row, col);

        if self.input_mode == InputMode::Notes && modifiers.shift {
            let start = primary_selection(&self.selected);
            select_range(&mut self.selected, start, position, &self.board);
        } else if self.input_mode == InputMode::Notes
            && modifiers.command
            && !self.board.cell(row, col).fixed
        {
            if let Some(index) = self
                .selected
                .iter()
                .position(|selected| *selected == position)
            {
                if self.selected.len() > 1 {
                    self.selected.remove(index);
                }
            } else {
                self.selected.push(position);
            }
        } else {
            self.selected.clear();
            self.selected.push(position);
        }
    }

    pub fn move_selection(&mut self, delta_row: i32, delta_col: i32) {
        if self.status == GameStatus::Finished {
            return;
        }

        let (row, col) = primary_selection(&self.selected).unwrap_or((0, 0));
        let next_row = (row as i32 + delta_row).clamp(0, 8) as usize;
        let next_col = (col as i32 + delta_col).clamp(0, 8) as usize;

        self.selected.clear();
        self.selected.push((next_row, next_col));
    }

    pub fn toggle_input_mode(&mut self) {
        if self.status == GameStatus::Finished {
            return;
        }

        self.input_mode = self.input_mode.toggle();
        if self.input_mode == InputMode::Value {
            collapse_to_primary_selection(&mut self.selected);
        }
    }

    pub fn place_digit(&mut self, value: u8) {
        if self.status == GameStatus::Finished {
            return;
        }

        if self.input_mode == InputMode::Notes {
            for &(row, col) in &self.selected {
                self.board.toggle_note(row, col, value);
            }
        } else if let Some((row, col)) = primary_selection(&self.selected) {
            if self.board.set_cell(row, col, Some(value)) {
                self.board.remove_related_notes(row, col, value);
            }
        }

        self.show_validation_result = false;
    }

    pub fn clear_selection(&mut self) {
        if !self.can_clear_selection() {
            return;
        }

        if self.input_mode == InputMode::Notes {
            for &(row, col) in &self.selected {
                self.board.clear_notes(row, col);
            }
        } else if let Some((row, col)) = primary_selection(&self.selected) {
            self.board.clear_value(row, col);
        }

        self.show_validation_result = false;
    }

    pub fn submit(&mut self, now_ms: u64) {
        if self.can_submit() {
            self.show_validation_result = true;
            self.status = GameStatus::Finished;
            self.finished_elapsed_ms = Some(
                self.started_at_ms
                    .map(|started_at_ms| now_ms.saturating_sub(started_at_ms))
                    .unwrap_or_default(),
            );
        }
    }

    pub fn view_state(&self, now_ms: u64) -> GameView {
        let selected_value = self.single_selected_value();
        let cells = (0..SIZE)
            .map(|row| {
                (0..SIZE)
                    .map(|col| self.cell_view(row, col, selected_value))
                    .collect()
            })
            .collect();

        GameView {
            started: self.started,
            status: self.status,
            input_mode: self.input_mode,
            validation_mode: self.validation_mode,
            difficulty: self.difficulty,
            cells,
            selected: self
                .selected
                .iter()
                .map(|&(row, col)| PositionView { row, col })
                .collect(),
            can_submit: self.can_submit(),
            can_clear: self.can_clear_selection(),
            show_complete: self.validation_mode == ValidationMode::Manual,
            recap: (self.status == GameStatus::Finished).then(|| self.recap(now_ms)),
        }
    }

    fn start_with_puzzle(&mut self, puzzle: Puzzle, now_ms: u64) {
        self.reset_with_puzzle(puzzle);
        self.started = true;
        self.started_at_ms = Some(now_ms);
    }

    fn reset_with_puzzle(&mut self, puzzle: Puzzle) {
        self.current_puzzle = puzzle;
        self.board = Board::from_start(puzzle);
        self.selected.clear();
        self.selected.push((0, 0));
        self.input_mode = InputMode::Value;
        self.show_validation_result = false;
        self.status = GameStatus::Playing;
        self.started_at_ms = None;
        self.finished_elapsed_ms = None;
    }

    fn cell_view(&self, row: usize, col: usize, selected_value: Option<u8>) -> CellView {
        let position = (row, col);
        let cell = self.board.cell(row, col);
        let selected = self.selected.contains(&position);
        let primary = primary_selection(&self.selected);
        let highlighted = primary
            .map(|(selected_row, selected_col)| selected_row == row || selected_col == col)
            .unwrap_or(false);
        let same_box = primary
            .map(|(selected_row, selected_col)| {
                selected_row / BOX_SIZE == row / BOX_SIZE
                    && selected_col / BOX_SIZE == col / BOX_SIZE
            })
            .unwrap_or(false);
        let same_value = selected_value.is_some() && cell.value == selected_value;
        let invalid = self.should_show_conflicts() && self.board.has_conflict(row, col);
        let notes = (0..SIZE)
            .filter_map(|index| cell.notes[index].then_some(index as u8 + 1))
            .collect();

        CellView {
            row,
            col,
            value: cell.value,
            notes,
            fixed: cell.fixed,
            selected,
            highlighted,
            same_box,
            same_value,
            invalid,
        }
    }

    fn should_show_conflicts(&self) -> bool {
        (self.status == GameStatus::Finished || self.input_mode == InputMode::Value)
            && match self.validation_mode {
                ValidationMode::Live => true,
                ValidationMode::Manual => self.show_validation_result,
            }
    }

    fn single_selected_value(&self) -> Option<u8> {
        if self.selected.len() == 1 {
            primary_selection(&self.selected).and_then(|(selected_row, selected_col)| {
                self.board.cell(selected_row, selected_col).value
            })
        } else {
            None
        }
    }

    fn can_submit(&self) -> bool {
        self.status == GameStatus::Playing
            && self.validation_mode == ValidationMode::Manual
            && self.board.is_filled()
    }

    fn can_clear_selection(&self) -> bool {
        if self.status == GameStatus::Finished {
            return false;
        }

        primary_selection(&self.selected)
            .map(|(row, col)| !self.board.cell(row, col).fixed)
            .unwrap_or(false)
    }

    fn recap(&self, now_ms: u64) -> RecapView {
        let elapsed_ms = self.finished_elapsed_ms.unwrap_or_else(|| {
            self.started_at_ms
                .map(|started_at_ms| now_ms.saturating_sub(started_at_ms))
                .unwrap_or_default()
        });

        RecapView {
            won: self.board.is_complete(),
            elapsed_ms,
            elapsed_label: format_duration(elapsed_ms),
            error_count: error_count(&self.board),
            filled_count: filled_count(&self.board),
        }
    }
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../../web/src/bindings/")]
pub struct GameView {
    pub started: bool,
    pub status: GameStatus,
    pub input_mode: InputMode,
    pub validation_mode: ValidationMode,
    pub difficulty: Difficulty,
    pub cells: Vec<Vec<CellView>>,
    pub selected: Vec<PositionView>,
    pub can_submit: bool,
    pub can_clear: bool,
    pub show_complete: bool,
    pub recap: Option<RecapView>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../../web/src/bindings/")]
pub struct PositionView {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../../web/src/bindings/")]
pub struct CellView {
    pub row: usize,
    pub col: usize,
    pub value: Option<u8>,
    pub notes: Vec<u8>,
    pub fixed: bool,
    pub selected: bool,
    pub highlighted: bool,
    pub same_box: bool,
    pub same_value: bool,
    pub invalid: bool,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../../web/src/bindings/")]
pub struct RecapView {
    pub won: bool,
    pub elapsed_ms: u64,
    pub elapsed_label: String,
    pub error_count: usize,
    pub filled_count: usize,
}

pub fn starter_puzzle() -> Puzzle {
    easy_puzzle()
}

pub fn easy_puzzle() -> Puzzle {
    generate_puzzle(46)
}

pub fn medium_puzzle() -> Puzzle {
    generate_puzzle(36)
}

pub fn hard_puzzle() -> Puzzle {
    generate_puzzle(26)
}

fn generate_puzzle(clues: usize) -> Puzzle {
    let target_clues = clues.min(SIZE * SIZE);
    let mut best = None;
    let mut best_clues = SIZE * SIZE + 1;

    for _ in 0..50 {
        let solved = generate_solution();
        let mut puzzle = solved.map(|row| row.map(Some));
        let mut current_clues = SIZE * SIZE;
        let mut positions: Vec<(usize, usize)> = (0..SIZE)
            .flat_map(|row| (0..SIZE).map(move |col| (row, col)))
            .collect();

        positions.shuffle(&mut thread_rng());

        for (row, col) in positions {
            if current_clues == target_clues {
                break;
            }

            let previous = puzzle[row][col];
            puzzle[row][col] = None;

            if solution_count(puzzle, 2) == 1 {
                current_clues -= 1;
            } else {
                puzzle[row][col] = previous;
            }
        }

        if current_clues == target_clues {
            return puzzle;
        }

        if current_clues < best_clues {
            best = Some(puzzle);
            best_clues = current_clues;
        }
    }

    best.unwrap_or_else(|| generate_solution().map(|row| row.map(Some)))
}

fn generate_solution() -> [[u8; SIZE]; SIZE] {
    let mut rng = thread_rng();

    let mut digits: Vec<u8> = (1..=9).collect();
    digits.shuffle(&mut rng);

    let rows = shuffled_units(&mut rng);
    let cols = shuffled_units(&mut rng);
    let mut solved = [[0; SIZE]; SIZE];

    for (target_row, source_row) in rows.iter().copied().enumerate() {
        for (target_col, source_col) in cols.iter().copied().enumerate() {
            let digit_index = pattern(source_row, source_col);
            solved[target_row][target_col] = digits[digit_index];
        }
    }

    solved
}

fn shuffled_units(rng: &mut impl rand::Rng) -> Vec<usize> {
    let mut bands: Vec<usize> = (0..BOX_SIZE).collect();
    bands.shuffle(rng);

    bands
        .into_iter()
        .flat_map(|band| {
            let mut units: Vec<usize> = (0..BOX_SIZE).collect();
            units.shuffle(rng);
            units
                .into_iter()
                .map(move |unit| band * BOX_SIZE + unit)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn pattern(row: usize, col: usize) -> usize {
    (BOX_SIZE * (row % BOX_SIZE) + row / BOX_SIZE + col) % SIZE
}

fn solution_count(puzzle: Puzzle, limit: usize) -> usize {
    let mut values = [[0; SIZE]; SIZE];

    for row in 0..SIZE {
        for col in 0..SIZE {
            values[row][col] = puzzle[row][col].unwrap_or(0);
        }
    }

    count_solutions(&mut values, limit)
}

fn count_solutions(values: &mut [[u8; SIZE]; SIZE], limit: usize) -> usize {
    let Some((row, col, candidates)) = best_empty_cell(values) else {
        return 1;
    };

    let mut count = 0;

    for value in candidates {
        values[row][col] = value;
        count += count_solutions(values, limit - count);
        values[row][col] = 0;

        if count >= limit {
            break;
        }
    }

    count
}

fn best_empty_cell(values: &[[u8; SIZE]; SIZE]) -> Option<(usize, usize, Vec<u8>)> {
    let mut best = None;

    for row in 0..SIZE {
        for col in 0..SIZE {
            if values[row][col] != 0 {
                continue;
            }

            let candidates = candidates(values, row, col);

            if candidates.is_empty() {
                return Some((row, col, candidates));
            }

            if best
                .as_ref()
                .map(|(_, _, best_candidates): &(usize, usize, Vec<u8>)| {
                    candidates.len() < best_candidates.len()
                })
                .unwrap_or(true)
            {
                best = Some((row, col, candidates));
            }
        }
    }

    best
}

fn candidates(values: &[[u8; SIZE]; SIZE], row: usize, col: usize) -> Vec<u8> {
    (1..=9)
        .filter(|value| can_place(values, row, col, *value))
        .collect()
}

fn can_place(values: &[[u8; SIZE]; SIZE], row: usize, col: usize, value: u8) -> bool {
    for current_col in 0..SIZE {
        if values[row][current_col] == value {
            return false;
        }
    }

    for current_row in values.iter().take(SIZE) {
        if current_row[col] == value {
            return false;
        }
    }

    let box_row = (row / BOX_SIZE) * BOX_SIZE;
    let box_col = (col / BOX_SIZE) * BOX_SIZE;
    for current_row in values.iter().skip(box_row).take(BOX_SIZE) {
        for current_value in current_row.iter().skip(box_col).take(BOX_SIZE) {
            if *current_value == value {
                return false;
            }
        }
    }

    true
}

fn primary_selection(selected: &[Position]) -> Option<Position> {
    selected.last().copied()
}

fn collapse_to_primary_selection(selected: &mut Vec<Position>) {
    if let Some(primary) = primary_selection(selected) {
        selected.clear();
        selected.push(primary);
    }
}

fn select_range(
    selected: &mut Vec<Position>,
    start: Option<Position>,
    end: Position,
    board: &Board,
) {
    let start = start.unwrap_or(end);
    let row_start = start.0.min(end.0);
    let row_end = start.0.max(end.0);
    let col_start = start.1.min(end.1);
    let col_end = start.1.max(end.1);

    selected.clear();

    for row in row_start..=row_end {
        for col in col_start..=col_end {
            if !board.cell(row, col).fixed {
                selected.push((row, col));
            }
        }
    }

    if selected.is_empty() {
        selected.push(end);
    }
}

fn filled_count(board: &Board) -> usize {
    let mut count = 0;

    for row in 0..SIZE {
        for col in 0..SIZE {
            if board.cell(row, col).value.is_some() {
                count += 1;
            }
        }
    }

    count
}

fn error_count(board: &Board) -> usize {
    let mut count = 0;

    for row in 0..SIZE {
        for col in 0..SIZE {
            if board.cell(row, col).value.is_some() && board.has_conflict(row, col) {
                count += 1;
            }
        }
    }

    count
}

fn format_duration(duration_ms: u64) -> String {
    let total_seconds = duration_ms / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    format!("{minutes}:{seconds:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_board() -> Board {
        Board::from_start([
            [
                Some(5),
                Some(3),
                None,
                None,
                Some(7),
                None,
                None,
                None,
                None,
            ],
            [
                Some(6),
                None,
                None,
                Some(1),
                Some(9),
                Some(5),
                None,
                None,
                None,
            ],
            [
                None,
                Some(9),
                Some(8),
                None,
                None,
                None,
                None,
                Some(6),
                None,
            ],
            [
                Some(8),
                None,
                None,
                None,
                Some(6),
                None,
                None,
                None,
                Some(3),
            ],
            [
                Some(4),
                None,
                None,
                Some(8),
                None,
                Some(3),
                None,
                None,
                Some(1),
            ],
            [
                Some(7),
                None,
                None,
                None,
                Some(2),
                None,
                None,
                None,
                Some(6),
            ],
            [
                None,
                Some(6),
                None,
                None,
                None,
                None,
                Some(2),
                Some(8),
                None,
            ],
            [
                None,
                None,
                None,
                Some(4),
                Some(1),
                Some(9),
                None,
                None,
                Some(5),
            ],
            [
                None,
                None,
                None,
                None,
                Some(8),
                None,
                None,
                Some(7),
                Some(9),
            ],
        ])
    }

    #[test]
    fn rejects_changes_to_fixed_cells() {
        let mut board = sample_board();
        assert!(!board.set_cell(0, 0, Some(1)));
        assert_eq!(board.cell(0, 0).value, Some(5));
    }

    #[test]
    fn accepts_valid_user_entry() {
        let mut board = sample_board();
        assert!(board.set_cell(0, 2, Some(4)));
        assert_eq!(board.cell(0, 2).value, Some(4));
        assert!(!board.has_conflict(0, 2));
    }

    #[test]
    fn detects_row_conflicts() {
        let mut board = sample_board();
        assert!(board.set_cell(0, 2, Some(5)));
        assert!(board.has_conflict(0, 2));
    }

    #[test]
    fn detects_column_conflicts() {
        let mut board = sample_board();
        assert!(board.set_cell(2, 0, Some(5)));
        assert!(board.has_conflict(2, 0));
    }

    #[test]
    fn detects_box_conflicts() {
        let mut board = sample_board();
        assert!(board.set_cell(1, 1, Some(9)));
        assert!(board.has_conflict(1, 1));
    }

    #[test]
    fn clears_only_user_cells() {
        let mut board = sample_board();
        assert!(board.set_cell(0, 2, Some(4)));
        assert!(board.toggle_note(0, 3, 2));
        board.clear_non_fixed();
        assert_eq!(board.cell(0, 2).value, None);
        assert!(board.cell(0, 3).note_values().is_empty());
        assert_eq!(board.cell(0, 0).value, Some(5));
    }

    #[test]
    fn toggles_notes_for_non_fixed_cells() {
        let mut board = sample_board();
        assert!(board.toggle_note(0, 2, 2));
        assert_eq!(board.cell(0, 2).note_values(), vec![2]);
        assert!(board.toggle_note(0, 2, 5));
        assert_eq!(board.cell(0, 2).note_values(), vec![2, 5]);
        assert!(board.toggle_note(0, 2, 2));
        assert_eq!(board.cell(0, 2).note_values(), vec![5]);
    }

    #[test]
    fn setting_value_clears_notes() {
        let mut board = sample_board();
        assert!(board.toggle_note(0, 2, 2));
        assert!(board.toggle_note(0, 2, 4));
        assert!(board.set_cell(0, 2, Some(4)));
        assert!(board.cell(0, 2).note_values().is_empty());
    }

    #[test]
    fn notes_do_not_replace_existing_values() {
        let mut board = sample_board();
        assert!(board.set_cell(0, 2, Some(4)));
        assert!(!board.toggle_note(0, 2, 2));
        assert_eq!(board.cell(0, 2).value, Some(4));
        assert!(board.cell(0, 2).note_values().is_empty());
    }

    #[test]
    fn removes_related_notes_for_placed_value() {
        let mut board = sample_board();
        assert!(board.toggle_note(0, 2, 4));
        assert!(board.toggle_note(0, 3, 4));
        assert!(board.toggle_note(1, 1, 4));
        assert!(board.toggle_note(4, 2, 4));
        assert!(board.toggle_note(4, 4, 4));

        board.remove_related_notes(0, 2, 4);

        assert!(board.cell(0, 2).note_values().is_empty());
        assert!(board.cell(0, 3).note_values().is_empty());
        assert!(board.cell(1, 1).note_values().is_empty());
        assert!(board.cell(4, 2).note_values().is_empty());
        assert_eq!(board.cell(4, 4).note_values(), vec![4]);
    }

    #[test]
    fn generated_puzzles_have_expected_clue_counts() {
        assert_eq!(clue_count(easy_puzzle()), 46);
        assert_eq!(clue_count(medium_puzzle()), 36);
        assert_eq!(clue_count(hard_puzzle()), 26);
    }

    #[test]
    fn generated_puzzles_do_not_have_conflicting_clues() {
        for puzzle in [easy_puzzle(), medium_puzzle(), hard_puzzle()] {
            assert_no_conflicts(puzzle);
        }
    }

    #[test]
    fn generated_puzzles_have_unique_solutions() {
        for puzzle in [easy_puzzle(), medium_puzzle(), hard_puzzle()] {
            assert_eq!(solution_count(puzzle, 2), 1);
        }
    }

    #[test]
    fn game_starts_and_finishes_with_recap() {
        let puzzle = [[Some(1); SIZE]; SIZE];
        let mut game = Game::default();
        game.start_with_puzzle(puzzle, 1_000);
        game.submit(5_000);
        let state = game.view_state(5_000);
        assert_eq!(state.status, GameStatus::Finished);
        assert_eq!(state.recap.unwrap().elapsed_label, "0:04");
    }

    #[test]
    fn export_bindings() {
        GameView::export_all().unwrap();
    }

    fn clue_count(puzzle: Puzzle) -> usize {
        puzzle
            .iter()
            .flat_map(|row| row.iter())
            .filter(|value| value.is_some())
            .count()
    }

    fn assert_no_conflicts(puzzle: Puzzle) {
        for row in 0..SIZE {
            for col in 0..SIZE {
                let Some(value) = puzzle[row][col] else {
                    continue;
                };

                for current_col in 0..SIZE {
                    assert!(
                        current_col == col || puzzle[row][current_col] != Some(value),
                        "row conflict at row {row}"
                    );
                }

                for current_row in 0..SIZE {
                    assert!(
                        current_row == row || puzzle[current_row][col] != Some(value),
                        "column conflict at column {col}"
                    );
                }

                let box_row = (row / BOX_SIZE) * BOX_SIZE;
                let box_col = (col / BOX_SIZE) * BOX_SIZE;
                for current_row in box_row..(box_row + BOX_SIZE) {
                    for current_col in box_col..(box_col + BOX_SIZE) {
                        assert!(
                            (current_row == row && current_col == col)
                                || puzzle[current_row][current_col] != Some(value),
                            "box conflict at row {row}, col {col}"
                        );
                    }
                }
            }
        }
    }
}
