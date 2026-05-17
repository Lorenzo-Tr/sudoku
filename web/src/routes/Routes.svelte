<script lang="ts">
  import type { GameView } from "../bindings/GameView";
  import LeaderboardPage from "../pages/leaderboard/LeaderboardPage.svelte";
  import PlayPage from "../pages/play/PlayPage.svelte";
  import PracticePage from "../pages/practice/PracticePage.svelte";
  import ProfilePage from "../pages/profile/ProfilePage.svelte";
  import SudokuSidebarControls from "../components/sudoku/SudokuSidebarControls.svelte";
  import type { AppRoute } from "./routes";

  export let route: AppRoute;
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
  export let userProfile = { name: "", handle: "", initials: "", streak: 0 };
  export let difficulty = "easy";
  export let liveMistakes = false;
  export let practiceElapsedLabel = "00:00";
  export let pendingDifficulty = "";
  export let confirmResetOpen = false;
  export let optionLabel: (option: string) => string = (option) => option;
  export let selectCell: (row: number, col: number, event: MouseEvent) => void = () => {};
  export let selectDaily: (day: number) => void = () => {};
  export let clearSelection: () => void = () => {};
  export let toggleNotes: () => void = () => {};
  export let placeDigit: (value: number) => void = () => {};
  export let submit: () => void = () => {};
  export let requestPracticeDifficulty: (option: string) => void = () => {};
  export let confirmPracticeDifficulty: () => void = () => {};
  export let cancelPracticeDifficulty: () => void = () => {};
  export let requestReset: () => void = () => {};
  export let reset: () => void = () => {};
  export let cancelReset: () => void = () => {};
  export let toggleLiveMistakes: () => void = () => {};
</script>

{#if route === "play"}
  <PlayPage
    {state}
    {selectedValue}
    {dailyLabel}
    {calendarMonth}
    {calendarWeekdays}
    {calendarDays}
    {selectedCalendarDay}
    {leaderboard}
    {streakIcon}
    {missedIcon}
    {selectCell}
    {selectDaily}
  >
    <SudokuSidebarControls
      slot="controls"
      className="daily-controls"
      canClear={state.canClear}
      canSubmit={state.canSubmit}
      inputMode={state.inputMode}
      showSubmit
      submitLabel="Submit daily"
      onClear={clearSelection}
      onToggleNotes={toggleNotes}
      onPlaceDigit={placeDigit}
      onSubmit={submit}
    />
  </PlayPage>
{:else if route === "practice"}
  <PracticePage
    {state}
    {selectedValue}
    {difficulty}
    {liveMistakes}
    {practiceElapsedLabel}
    {pendingDifficulty}
    {confirmResetOpen}
    {optionLabel}
    {selectCell}
    {requestPracticeDifficulty}
    {confirmPracticeDifficulty}
    {cancelPracticeDifficulty}
    {requestReset}
    {reset}
    {cancelReset}
    {toggleLiveMistakes}
    {clearSelection}
    {toggleNotes}
    {placeDigit}
    {submit}
  />
{:else if route === "leaderboard"}
  <LeaderboardPage {leaderboard} />
{:else}
  <ProfilePage {userProfile} {streakIcon} />
{/if}
