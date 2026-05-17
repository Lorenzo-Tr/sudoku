<script lang="ts">
  import type { CellView } from "../../bindings/CellView";

  export let cells: CellView[][] = [];
  export let selectedValue: number | null = null;
  export let className = "";
  export let onSelect: (row: number, col: number, event: MouseEvent) => void = () => {};
</script>

<div class={`sudoku-board ${className}`} aria-label="Sudoku board">
  {#each cells as row}
    <div class="sudoku-row">
      {#each row as cell}
        <button
          class="sudoku-cell"
          class:selected={cell.selected}
          class:invalid={cell.invalid}
          class:sameValue={cell.sameValue}
          class:highlighted={cell.highlighted}
          class:same-box={cell.sameBox}
          class:fixed={cell.fixed}
          class:box-right={cell.col === 2 || cell.col === 5}
          class:last-col={cell.col === 8}
          class:box-bottom={cell.row === 2 || cell.row === 5}
          class:last-row={cell.row === 8}
          onclick={(event) => onSelect(cell.row, cell.col, event)}
        >
          {#if cell.value}
            <span class="value">{cell.value}</span>
          {:else}
            <span class="notes">
              {#each Array.from({ length: 9 }, (_, index) => index + 1) as note}
                <span
                  class:active-note={cell.notes.includes(note)}
                  class:matching-note={selectedValue === note && cell.notes.includes(note)}
                >
                  {cell.notes.includes(note) ? note : ""}
                </span>
              {/each}
            </span>
          {/if}
        </button>
      {/each}
    </div>
  {/each}
</div>

<style>
  :global(.sudoku-board) {
    display: grid;
    grid-template-rows: repeat(9, minmax(0, 1fr));
    aspect-ratio: 1 / 1;
    width: 100%;
    overflow: hidden;
    border: 2px solid var(--setup-text, #193b35);
    background: var(--setup-text, #193b35);
    box-shadow: 0 18px 48px rgb(25 72 61 / 12%);
  }

  :global(.sudoku-row) {
    display: grid;
    grid-template-columns: repeat(9, minmax(0, 1fr));
    min-width: 0;
    min-height: 0;
  }

  :global(.sudoku-cell) {
    appearance: none;
    display: grid;
    place-items: center;
    min-width: 0;
    width: 100%;
    min-height: 0;
    height: 100%;
    margin: 0;
    padding: 0;
    outline: 0;
    border: 0;
    border-right: 1px solid var(--setup-text, #193b35);
    border-bottom: 1px solid var(--setup-text, #193b35);
    border-radius: 0;
    color: var(--setup-text, #193b35);
    background: var(--setup-surface, #fbfffd);
    box-shadow: none;
    font-size: clamp(18px, 3vw, 34px);
    font-weight: 700;
    line-height: 1;
    cursor: pointer;
  }

  :global(.sudoku-cell:focus-visible) {
    box-shadow: inset 0 0 0 3px rgb(47 125 101 / 36%);
  }

  :global(.sudoku-cell.box-right) {
    border-right-width: 2px;
  }

  :global(.sudoku-cell.last-col) {
    border-right: 0;
  }

  :global(.sudoku-cell.box-bottom) {
    border-bottom-width: 2px;
  }

  :global(.sudoku-cell.last-row) {
    border-bottom: 0;
  }

  :global(.sudoku-cell.same-box) {
    background: #e3f0e9;
  }

  :global(.sudoku-cell.highlighted) {
    background: #d8eadf;
  }

  :global(.sudoku-cell.sameValue) {
    background: #c6dfd0;
  }

  :global(.sudoku-cell.selected) {
    background: #afd4bf;
  }

  :global(.sudoku-cell.invalid) {
    background: #f7c7c7;
  }

  :global(.sudoku-cell:not(.fixed) .value) {
    color: var(--setup-primary, #2f7d65);
  }

  :global(.sudoku-cell .value) {
    display: block;
    font-size: inherit;
    line-height: 1;
  }

  :global(.sudoku-cell .notes) {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1px 2px;
    width: 100%;
    height: 100%;
    padding: 8px 6px;
    color: #7f968e;
    font-size: 12px;
  }

  :global(.sudoku-cell .active-note) {
    color: var(--setup-primary, #2f7d65);
    font-weight: 700;
  }

  :global(.sudoku-cell .matching-note) {
    color: var(--setup-primary, #2f7d65);
    font-weight: 800;
    text-decoration: underline;
    text-underline-offset: 2px;
  }
</style>
