# Tauri WASM Migration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Migrate the Sudoku app from an `iced` desktop UI to a web-first Svelte UI packaged by Tauri, with all game logic owned by Rust and exposed to the UI through WASM.

**Architecture:** Convert the repo to a Cargo workspace. Move board, puzzle generation, rules, game session state, and recap stats into `crates/sudoku-core`. Add `crates/sudoku-wasm` as a thin `wasm-bindgen` wrapper that exposes explicit typed methods and returns serialized JS objects via `serde-wasm-bindgen`. Build a Vite + Svelte app in `web/`, then wrap it with Tauri in `src-tauri/`.

**Tech Stack:** Rust 2024, wasm-bindgen, serde-wasm-bindgen, ts-rs, Vite, Svelte, Tauri v2.

---

### Task 1: Workspace and Core

**Files:**
- Modify: `Cargo.toml`
- Create: `crates/sudoku-core/Cargo.toml`
- Create: `crates/sudoku-core/src/lib.rs`

**Steps:**
1. Make the repository a Cargo workspace.
2. Move board and puzzle logic into `sudoku-core`.
3. Add a `Game` session that owns state and exposes action methods.
4. Add serializable view structs for frontend rendering.
5. Preserve existing tests and add game-level tests.

### Task 2: WASM Wrapper

**Files:**
- Create: `crates/sudoku-wasm/Cargo.toml`
- Create: `crates/sudoku-wasm/src/lib.rs`

**Steps:**
1. Wrap `sudoku_core::Game` with `wasm-bindgen`.
2. Expose explicit methods: `start`, `select_cell`, `place_digit`, `clear_selection`, `toggle_notes`, `submit`, `retry`, `new_game`, `state`.
3. Use `serde-wasm-bindgen` for state transfer.
4. Generate TypeScript types from Rust view structs.

### Task 3: Web UI

**Files:**
- Create: `web/`

**Steps:**
1. Add Vite + Svelte.
2. Build the setup screen, grid, side panel, keypad, and recap screen from `GameView`.
3. Forward UI events to the WASM `Game` methods.
4. Keep platform-specific logic out of the game UI.

### Task 4: Tauri Shell

**Files:**
- Create: `src-tauri/`

**Steps:**
1. Add Tauri v2 config.
2. Point Tauri to the Vite dev server and built web assets.
3. Package the same web UI for desktop.

### Task 5: Verification

**Commands:**
- `cargo test --workspace`
- `pnpm install`
- `pnpm --dir web build`
- `pnpm --dir web tauri build` if available locally

**Expected:** Rust tests pass, web app builds, and Tauri project is ready for CI packaging.
