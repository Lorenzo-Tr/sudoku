# Sudoku Setup Screen Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a dedicated setup screen with difficulty and submit mode selection before showing the Sudoku game board.

**Architecture:** Keep the existing `SudokuApp` state and use `started` as the view switch. Add a `StartGamePressed` message, split view rendering into setup and game helpers, and rebuild the game side panel without difficulty controls or setup hints.

**Tech Stack:** Rust 2024, iced 0.13.

---

### Task 1: Add Setup Flow

**Files:**
- Modify: `src/app.rs`

**Steps:**
1. Add `StartGamePressed` to `Message`.
2. Handle it by resetting the board, selection, and validation result, then setting `started = true`.
3. Change `view` to render setup when `started` is false and game when true.
4. Add a setup screen with difficulty buttons, submit mode buttons, and Start game button.

### Task 2: Rebuild Game Side Panel

**Files:**
- Modify: `src/app.rs`

**Steps:**
1. Remove difficulty controls and setup copy from the game side panel.
2. Add compact tool controls for reset, clear/erase, note/value mode, submit, and new game.
3. Increase keypad button size and spacing to match the provided visual direction.

### Task 3: Verify

**Commands:**
- `cargo fmt`
- `cargo test`

**Expected:** formatting succeeds and all tests pass.
