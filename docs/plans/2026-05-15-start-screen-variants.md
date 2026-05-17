# Start Screen Variants Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add six in-app selectable start-screen visual variants for live comparison in dev mode.

**Architecture:** Keep one setup-state data flow in `App.svelte`; add a local `setupVariant` UI-only state and use CSS variant classes for layout differences. The same difficulty, validation mode, and start handlers are shared by all variants.

**Tech Stack:** Svelte 5, Vite, CSS.

---

### Task 1: Add Variant Switcher and Shared Setup Markup

**Files:**
- Modify: `web/src/App.svelte`

**Steps:**
1. Add `setupVariant` state and arrays for variant numbers and mini-board preview cells.
2. Replace the setup branch with a variant switcher and a shared setup panel.
3. Keep all existing handlers and state unchanged.

### Task 2: Add Six Visual Layouts

**Files:**
- Modify: `web/src/styles.css`

**Steps:**
1. Add variant switcher styling.
2. Add `.setup-v1` through `.setup-v6` layout rules.
3. Add preview board and responsive rules.

### Task 3: Verify

**Commands:**
- `pnpm build:web` should pass.
- `cargo test --workspace --locked` should pass.
