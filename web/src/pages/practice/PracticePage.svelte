<script lang="ts">
  import "./practice.css";
  import type { GameView } from "../../bindings/GameView";
  import SudokuBoard from "../../components/sudoku/SudokuBoard.svelte";
  import SudokuSidebarControls from "../../components/sudoku/SudokuSidebarControls.svelte";

  export let state: GameView;
  export let selectedValue: number | null = null;
  export let difficulty = "easy";
  export let liveMistakes = false;
  export let practiceElapsedLabel = "00:00";
  export let pendingDifficulty = "";
  export let confirmResetOpen = false;
  export let optionLabel: (option: string) => string = (option) => option;
  export let selectCell: (row: number, col: number, event: MouseEvent) => void = () => {};
  export let requestPracticeDifficulty: (option: string) => void = () => {};
  export let confirmPracticeDifficulty: () => void = () => {};
  export let cancelPracticeDifficulty: () => void = () => {};
  export let requestReset: () => void = () => {};
  export let reset: () => void = () => {};
  export let cancelReset: () => void = () => {};
  export let toggleLiveMistakes: () => void = () => {};
  export let clearSelection: () => void = () => {};
  export let toggleNotes: () => void = () => {};
  export let placeDigit: (value: number) => void = () => {};
  export let submit: () => void = () => {};
</script>

<section class="practice-panel practice-play">
  <div class="practice-stage">
    <div class="practice-board-card">
      <SudokuBoard
        cells={state.cells}
        {selectedValue}
        className="practice-board"
        onSelect={selectCell}
      />
    </div>

    <aside class="practice-action-panel">
      <SudokuSidebarControls
        className="practice-sidebar"
        canClear={state.canClear}
        canSubmit={state.canSubmit}
        inputMode={state.inputMode}
        showHeader
        elapsedLabel={practiceElapsedLabel}
        {difficulty}
        {liveMistakes}
        {optionLabel}
        showSubmit
        submitLabel="Submit"
        onDifficultyChange={requestPracticeDifficulty}
        onReset={requestReset}
        onToggleLiveMistakes={toggleLiveMistakes}
        onClear={clearSelection}
        onToggleNotes={toggleNotes}
        onPlaceDigit={placeDigit}
        onSubmit={submit}
      />
    </aside>
  </div>

  {#if pendingDifficulty}
    <div class="dialog-backdrop" role="presentation">
      <div class="confirm-dialog" role="dialog" aria-modal="true" aria-labelledby="difficulty-dialog-title">
        <h2 id="difficulty-dialog-title">Start new game</h2>
        <p>Changing difficulty will lose your current progress.</p>
        <div class="dialog-actions">
          <button on:click={cancelPracticeDifficulty}>Cancel</button>
          <button class="primary" on:click={confirmPracticeDifficulty}>Confirm</button>
        </div>
      </div>
    </div>
  {/if}

  {#if confirmResetOpen}
    <div class="dialog-backdrop" role="presentation">
      <div class="confirm-dialog" role="dialog" aria-modal="true" aria-labelledby="reset-dialog-title">
        <h2 id="reset-dialog-title">Reset puzzle</h2>
        <p>This will clear your current progress and restart the timer.</p>
        <div class="dialog-actions">
          <button on:click={cancelReset}>Cancel</button>
          <button class="danger-confirm" on:click={reset}>Reset puzzle</button>
        </div>
      </div>
    </div>
  {/if}
</section>
