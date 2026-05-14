# Sudoku Setup Screen Design

## Goal

Add a dedicated pre-game setup screen for choosing difficulty and submit mode, then show the Sudoku grid with a focused right-side control panel after the game starts.

## Approach

The app keeps its existing single-state `SudokuApp` structure and uses the existing `started` flag to choose between setup and game views. The setup screen shows difficulty choices, validation mode choices, and a Start game button. The game screen removes difficulty controls and setup hints, keeping the board plus tools and keypad.

## Game Screen

The right panel follows the provided reference at a smaller desktop-app scale: a row of rounded tool buttons, a large 3x3 number pad, and a prominent New game button. It excludes hint, error count, timer, and difficulty controls.

## Behavior

Pressing Start game creates a fresh board using the selected setup options. Pressing New game returns to setup so the player can choose difficulty and submit mode again.
