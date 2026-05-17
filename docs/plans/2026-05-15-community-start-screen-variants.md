# Community Start Screen Variants Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the setup-only start screen with six distinct community-oriented frontend concepts.

**Architecture:** Keep all behavior local to `App.svelte`; the daily, account, stats, and leaderboard content are frontend placeholders. Existing game start logic remains unchanged, with Daily using the current selected difficulty/mode and Practice retaining explicit selection.

**Tech Stack:** Svelte 5, Vite, CSS.

---

### Task 1: Add community placeholder data and start actions

- Modify `web/src/App.svelte` to add daily stats, leaderboard rows, and variant metadata.

### Task 2: Replace setup markup with daily hub markup

- Modify the setup branch to render account, daily challenge, leaderboard, stats, and practice settings.

### Task 3: Add six distinct visual themes

- Modify `web/src/styles.css` with variant-specific palettes, typography, surfaces, and layout.

### Task 4: Verify

- Run `pnpm build:web` and `cargo test --workspace --locked`.
