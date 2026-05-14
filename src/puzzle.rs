use rand::seq::SliceRandom;
use rand::thread_rng;

const SIZE: usize = 9;
const BOX_SIZE: usize = 3;

pub type Puzzle = [[Option<u8>; SIZE]; SIZE];

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

    for current_row in 0..SIZE {
        if values[current_row][col] == value {
            return false;
        }
    }

    let box_row = (row / BOX_SIZE) * BOX_SIZE;
    let box_col = (col / BOX_SIZE) * BOX_SIZE;
    for current_row in box_row..(box_row + BOX_SIZE) {
        for current_col in box_col..(box_col + BOX_SIZE) {
            if values[current_row][current_col] == value {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{easy_puzzle, hard_puzzle, medium_puzzle};

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
            assert_eq!(super::solution_count(puzzle, 2), 1);
        }
    }

    fn clue_count(puzzle: super::Puzzle) -> usize {
        puzzle
            .iter()
            .flat_map(|row| row.iter())
            .filter(|value| value.is_some())
            .count()
    }

    fn assert_no_conflicts(puzzle: super::Puzzle) {
        for row in 0..9 {
            for col in 0..9 {
                let Some(value) = puzzle[row][col] else {
                    continue;
                };

                for current_col in 0..9 {
                    assert!(
                        current_col == col || puzzle[row][current_col] != Some(value),
                        "row conflict at row {row}"
                    );
                }

                for current_row in 0..9 {
                    assert!(
                        current_row == row || puzzle[current_row][col] != Some(value),
                        "column conflict at column {col}"
                    );
                }

                let box_row = (row / 3) * 3;
                let box_col = (col / 3) * 3;
                for current_row in box_row..(box_row + 3) {
                    for current_col in box_col..(box_col + 3) {
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
