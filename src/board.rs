#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub value: Option<u8>,
    pub notes: [bool; 9],
    pub fixed: bool,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            value: None,
            notes: [false; 9],
            fixed: false,
        }
    }

    pub fn fixed(value: u8) -> Self {
        Self {
            value: Some(value),
            notes: [false; 9],
            fixed: true,
        }
    }

    #[cfg(test)]
    pub fn note_values(&self) -> Vec<u8> {
        self.notes
            .iter()
            .enumerate()
            .filter_map(|(index, enabled)| enabled.then_some(index as u8 + 1))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    cells: [[Cell; 9]; 9],
}

impl Board {
    pub fn from_start(start: [[Option<u8>; 9]; 9]) -> Self {
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
        for row in 0..9 {
            for col in 0..9 {
                if !self.cells[row][col].fixed {
                    self.cells[row][col].value = None;
                    self.cells[row][col].notes = [false; 9];
                }
            }
        }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, value: Option<u8>) -> bool {
        if row >= 9 || col >= 9 {
            return false;
        }

        if self.cells[row][col].fixed {
            return false;
        }

        if let Some(value) = value {
            if !(1..=9).contains(&value) {
                return false;
            }
        }

        self.cells[row][col].value = value;
        self.cells[row][col].notes = [false; 9];
        true
    }

    pub fn clear_value(&mut self, row: usize, col: usize) -> bool {
        self.set_cell(row, col, None)
    }

    pub fn toggle_note(&mut self, row: usize, col: usize, value: u8) -> bool {
        if row >= 9 || col >= 9 || !(1..=9).contains(&value) {
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
        if row >= 9 || col >= 9 || self.cells[row][col].fixed {
            return false;
        }

        self.cells[row][col].notes = [false; 9];
        true
    }

    pub fn remove_related_notes(&mut self, row: usize, col: usize, value: u8) {
        if row >= 9 || col >= 9 || !(1..=9).contains(&value) {
            return;
        }

        let index = (value - 1) as usize;
        self.cells[row][col].notes[index] = false;

        for current_col in 0..9 {
            if current_col != col && !self.cells[row][current_col].fixed {
                self.cells[row][current_col].notes[index] = false;
            }
        }

        for current_row in 0..9 {
            if current_row != row && !self.cells[current_row][col].fixed {
                self.cells[current_row][col].notes[index] = false;
            }
        }

        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for current_row in box_row..(box_row + 3) {
            for current_col in box_col..(box_col + 3) {
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

        for current_col in 0..9 {
            if current_col != col && self.cells[row][current_col].value == Some(value) {
                return true;
            }
        }

        for current_row in 0..9 {
            if current_row != row && self.cells[current_row][col].value == Some(value) {
                return true;
            }
        }

        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for current_row in box_row..(box_row + 3) {
            for current_col in box_col..(box_col + 3) {
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
        for row in 0..9 {
            for col in 0..9 {
                if self.cells[row][col].value.is_none() || self.has_conflict(row, col) {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_filled(&self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.cells[row][col].value.is_none() {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

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
}
