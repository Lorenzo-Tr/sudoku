<script lang="ts">
  import ControlButton from "./SudokuControlButton.svelte";
  import resetIcon from "../../assets/rotate-ccw.svg?raw";
  import eraserIcon from "../../assets/eraser.svg?raw";
  import notesIcon from "../../assets/notebook-pen.svg?raw";

  export let canClear = false;
  export let canSubmit = false;
  export let inputMode = "";
  export let showReset = false;
  export let showSubmit = false;
  export let submitLabel = "Submit";
  export let primaryLabel = "";
  export let primaryDisabled = false;
  export let className = "";
  export let showHeader = false;
  export let elapsedLabel = "00:00";
  export let difficulty = "easy";
  export let liveMistakes = false;
  export let optionLabel: (option: string) => string = (option) => option;
  export let onDifficultyChange: (option: string) => void = () => {};
  export let onToggleLiveMistakes: () => void = () => {};
  export let onReset: () => void = () => {};
  export let onClear: () => void = () => {};
  export let onToggleNotes: () => void = () => {};
  export let onPlaceDigit: (value: number) => void = () => {};
  export let onSubmit: () => void = () => {};
  export let onPrimary: () => void = () => {};

  const digits = Array.from({ length: 9 }, (_, index) => index + 1);
</script>

<section class={`sudoku-sidebar-controls ${className}`}>
  {#if showHeader}
    <div class="practice-side-header">
      <div>
        <span>Time</span>
        <strong>{elapsedLabel}</strong>
      </div>

      <label>
        <span>Difficulty</span>
        <select
          value={difficulty}
          onchange={(event) => onDifficultyChange((event.currentTarget as HTMLSelectElement).value)}
        >
          {#each ["easy", "medium", "hard"] as option}
            <option value={option}>{optionLabel(option)}</option>
          {/each}
        </select>
      </label>

      <button class="practice-reset-button" onclick={() => onReset()}>Reset</button>
    </div>

    <label class="mistake-toggle">
      <span>Live mistakes</span>
      <button
        type="button"
        role="switch"
        aria-label="Toggle live mistakes"
        aria-checked={liveMistakes}
        class:active={liveMistakes}
        onclick={() => onToggleLiveMistakes()}
      >
        <span></span>
      </button>
    </label>
  {/if}

  <div class="controls-container">
    <div class="tools">
      {#if showReset}
        <ControlButton label="Reset" icon={resetIcon} onPress={onReset} />
      {/if}
      <ControlButton label="Delete" icon={eraserIcon} disabled={!canClear} onPress={onClear} />
      <ControlButton
        label="Notes"
        icon={notesIcon}
        active={inputMode === "notes"}
        onPress={onToggleNotes}
      />
    </div>

    <div class="keypad" aria-label="Number keypad">
      {#each digits as value}
        <button type="button" onclick={() => onPlaceDigit(value)}>{value}</button>
      {/each}
    </div>

  </div>

  {#if showSubmit}
    <button class="primary sidebar-submit" disabled={!canSubmit} onclick={() => onSubmit()}>
      {submitLabel}
    </button>
  {/if}

  {#if primaryLabel}
    <button class="primary sidebar-primary" disabled={primaryDisabled} onclick={() => onPrimary()}>
      {primaryLabel}
    </button>
  {/if}
</section>

<style>
  .controls-container {
    display: grid;
    gap: 10px;
  }
</style>
