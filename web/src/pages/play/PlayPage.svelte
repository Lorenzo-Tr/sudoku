<script lang="ts">
  import "./play.css";
  import type { GameView } from "../../bindings/GameView";
  import SudokuBoard from "../../components/sudoku/SudokuBoard.svelte";

  export let state: GameView;
  export let selectedValue: number | null = null;
  export let dailyLabel = "Today";
  export let calendarMonth = "";
  export let calendarWeekdays: string[] = [];
  export let calendarDays: Array<{ day: number; status: string } | null> = [];
  export let selectedCalendarDay = 0;
  export let leaderboard: Array<{ rank: number; name: string; time: string; score: string }> = [];
  export let streakIcon = "";
  export let missedIcon = "";
  export let selectCell: (row: number, col: number, event: MouseEvent) => void = () => {};
  export let selectDaily: (day: number) => void = () => {};
</script>

<div class="play-layout">
  <section class="daily-play">
    <div class="daily-board-shell">
      <div class="daily-toolbar">
        <div>
          <span class="daily-kicker">Daily Sudoku</span>
          <strong>{dailyLabel}</strong>
        </div>
      </div>

      <div class="daily-board-row">
        <SudokuBoard cells={state.cells} {selectedValue} onSelect={selectCell} />

        <aside class="daily-side">
          {#if state.status === "finished"}
            <section class="leaderboard-panel">
              <div class="section-title">Today&apos;s ranking</div>
              {#each leaderboard as player}
                <div class:you={player.name === "You"} class="leader-row">
                  <span>{player.rank}</span>
                  <strong>{player.name}</strong>
                  <em>{player.time}</em>
                  <small>{player.score}</small>
                </div>
              {/each}
            </section>
          {/if}

          <slot name="controls" />
        </aside>
      </div>
    </div>
  </section>
</div>

<section class="daily-calendar-section">
  <div class="calendar-header">
    <span class="section-title">Daily archive</span>
    <div class="calendar-heading-row">
      <h2>{calendarMonth}</h2>
      <div class="month-controls">
        <button>Previous month</button>
        <button disabled>Next month</button>
      </div>
    </div>
  </div>

  <div class="month-calendar calendar-soft-calm" aria-label="Daily Sudoku calendar">
    {#each calendarWeekdays as weekday}
      <div class="weekday">{weekday}</div>
    {/each}

    {#each calendarDays as day}
      {#if day}
        <button
          class:active={selectedCalendarDay === day.day}
          class:played={day.status === "played"}
          class:missed={day.status === "missed"}
          class:current={day.status === "current"}
          class:future={day.status === "future"}
          disabled={day.status === "future"}
          on:click={() => selectDaily(day.day)}
        >
          <span>{day.day}</span>
          {#if day.status === "played"}
            {@html streakIcon}
          {:else if day.status === "missed"}
            {@html missedIcon}
          {:else if day.status === "current"}
            <strong>Today</strong>
          {/if}
        </button>
      {:else}
        <div class="calendar-empty"></div>
      {/if}
    {/each}
  </div>
</section>
