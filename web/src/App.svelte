<script lang="ts">
  import init, { Game } from "./wasm/sudoku_wasm";
  import type { CellView } from "./bindings/CellView";
  import type { GameView } from "./bindings/GameView";
  import resetIcon from "./assets/rotate-ccw.svg?raw";
  import eraserIcon from "./assets/eraser.svg?raw";
  import notesIcon from "./assets/notebook-pen.svg?raw";

  let game: Game | null = null;
  let state: GameView | null = null;
  let ready = false;
  let difficulty = "easy";
  let validationMode = "manual";

  init().then(() => {
    game = new Game();
    refresh();
    ready = true;
  });

  function refresh() {
    if (!game) return;
    state = game.state() as GameView;
  }

  function start() {
    game?.start(difficulty, validationMode);
    refresh();
  }

  function selectCell(row: number, col: number, event: MouseEvent) {
    game?.select_cell(row, col, event.shiftKey, event.metaKey || event.ctrlKey);
    refresh();
  }

  function placeDigit(value: number) {
    game?.place_digit(value);
    refresh();
  }

  function clearSelection() {
    game?.clear_selection();
    refresh();
  }

  function toggleNotes() {
    game?.toggle_notes();
    refresh();
  }

  function reset() {
    game?.reset();
    refresh();
  }

  function submit() {
    game?.submit();
    refresh();
  }

  function retry() {
    game?.retry();
    refresh();
  }

  function newGame() {
    game?.new_game();
    refresh();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!game || !state?.started || state.status === "finished") return;

    if (/^[1-9]$/.test(event.key)) {
      game.place_digit(Number(event.key));
    } else if (event.key === "Backspace" || event.key === "Delete") {
      game.clear_selection();
    } else if (event.key === " " || event.key.toLowerCase() === "n") {
      event.preventDefault();
      game.toggle_notes();
    } else if (event.key === "Enter") {
      game.submit();
    } else if (event.key === "ArrowUp") {
      game.move_selection(-1, 0);
    } else if (event.key === "ArrowDown") {
      game.move_selection(1, 0);
    } else if (event.key === "ArrowLeft") {
      game.move_selection(0, -1);
    } else if (event.key === "ArrowRight") {
      game.move_selection(0, 1);
    } else {
      return;
    }

    refresh();
  }

  function cellClass(cell: CellView) {
    return {
      selected: cell.selected,
      invalid: cell.invalid,
      sameValue: cell.sameValue,
      highlighted: cell.highlighted,
      sameBox: cell.sameBox,
      fixed: cell.fixed,
    };
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if !ready || !state}
  <main class="app loading">Loading</main>
{:else if !state.started}
  <main class="app setup-shell">
    <section class="setup-panel">
      <h1>Sudoku</h1>
      <p>Choose your difficulty and submit mode</p>

      <div class="field-label">Difficulty</div>
      <div class="segmented three">
        {#each ["easy", "medium", "hard"] as option}
          <button class:active={difficulty === option} on:click={() => (difficulty = option)}>
            {option}
          </button>
        {/each}
      </div>

      <div class="field-label">Submit mode</div>
      <div class="segmented two">
        <button class:active={validationMode === "live"} on:click={() => (validationMode = "live")}>
          <span>A</span>
          <small>Auto</small>
        </button>
        <button
          class:active={validationMode === "manual"}
          on:click={() => (validationMode = "manual")}
        >
          <span>✓</span>
          <small>Submit</small>
        </button>
      </div>

      <button class="primary" on:click={start}>Start game</button>
    </section>
  </main>
{:else}
  <main class="app game-shell">
    <section class="board" aria-label="Sudoku board">
      {#each state.cells as row}
        {#each row as cell}
          <button
            class="cell"
            class:selected={cellClass(cell).selected}
            class:invalid={cellClass(cell).invalid}
            class:sameValue={cellClass(cell).sameValue}
            class:highlighted={cellClass(cell).highlighted}
            class:same-box={cellClass(cell).sameBox}
            class:fixed={cellClass(cell).fixed}
            style={`--row: ${cell.row}; --col: ${cell.col}`}
            on:click={(event) => selectCell(cell.row, cell.col, event)}
          >
            {#if cell.value}
              <span class="value">{cell.value}</span>
            {:else}
              <span class="notes">
                {#each Array.from({ length: 9 }, (_, index) => index + 1) as note}
                  <span class:active-note={cell.notes.includes(note)}>{cell.notes.includes(note) ? note : ""}</span>
                {/each}
              </span>
            {/if}
          </button>
        {/each}
      {/each}
    </section>

    <aside class="side-panel">
      {#if state.status === "finished" && state.recap}
        <section class="recap">
          <h2 class:lost={!state.recap.won}>{state.recap.won ? "Win" : "Defeat"}</h2>
          <div><span>Time</span><strong>{state.recap.elapsedLabel}</strong></div>
          <div><span>Errors</span><strong>{state.recap.errorCount}</strong></div>
          <div><span>Filled</span><strong>{state.recap.filledCount}/81</strong></div>
        </section>
        <div class="spacer"></div>
        <button class="primary" on:click={retry}>Retry</button>
        <button class="primary" on:click={newGame}>New game</button>
      {:else}
        {#if state.showComplete}
          <button class="primary" disabled={!state.canSubmit} on:click={submit}>Complete</button>
        {/if}

        <div class="tools">
          <button aria-label="Reset" on:click={reset}>{@html resetIcon}</button>
          <button aria-label="Delete" disabled={!state.canClear} on:click={clearSelection}>{@html eraserIcon}</button>
          <button
            aria-label="Notes"
            class:active={state.inputMode === "notes"}
            on:click={toggleNotes}
          >
            {@html notesIcon}
          </button>
        </div>

        <div class="keypad">
          {#each Array.from({ length: 9 }, (_, index) => index + 1) as value}
            <button on:click={() => placeDigit(value)}>{value}</button>
          {/each}
        </div>

        <div class="spacer"></div>
        <button class="primary" on:click={newGame}>New game</button>
      {/if}
    </aside>
  </main>
{/if}
