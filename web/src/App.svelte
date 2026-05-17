<script lang="ts">
  import init, { Game } from "./wasm/sudoku_wasm";
  import wasmUrl from "./wasm/sudoku_wasm_bg.wasm?url";
  import type { CellView } from "./bindings/CellView";
  import type { GameView } from "./bindings/GameView";
  import Routes from "./routes/Routes.svelte";
  import { navRoutes, pathForRoute, routeFromPath, type AppRoute } from "./routes/routes";
  import SudokuSidebarControls from "./components/sudoku/SudokuSidebarControls.svelte";
  import streakIcon from "./assets/flame.svg?raw";
  import missedIcon from "./assets/x.svg?raw";

  let game: Game | null = null;
  let state: GameView | null = null;
  let ready = false;
  let startupError = "";
  const initialParams = new URLSearchParams(window.location.search);
  let difficulty = parseDifficulty(initialParams.get("difficulty"));
  let validationMode = initialParams.get("liveMistakes") === "1" ? "live" : "manual";
  let homeTab: AppRoute = routeFromPath(window.location.pathname);
  let pendingDifficulty = "";
  let practiceStartedAt = Date.now();
  let practiceTimerRunning = false;
  let confirmResetOpen = false;
  let elapsedSeconds = 0;
  let dailyOffset = 0;
  let homeMode = true;
  let navScrolled = false;

  const userProfile = {
    name: "Lorenzo",
    handle: "@lorenzo",
    initials: "LT",
    streak: 0,
  };
  const leaderboard = [
    { rank: 1, name: "Maya", time: "04:12", score: "982" },
    { rank: 2, name: "Theo", time: "04:48", score: "941" },
    { rank: 3, name: "Nora", time: "05:03", score: "917" },
    { rank: 42, name: "You", time: "--:--", score: "Unplayed" },
  ];
  const calendarWeekdays = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
  const calendarMonth = "May 2026";
  const calendarDays = [
    null,
    null,
    null,
    null,
    { day: 1, status: "missed" },
    { day: 2, status: "played" },
    { day: 3, status: "played" },
    { day: 4, status: "missed" },
    { day: 5, status: "played" },
    { day: 6, status: "played" },
    { day: 7, status: "played" },
    { day: 8, status: "missed" },
    { day: 9, status: "played" },
    { day: 10, status: "played" },
    { day: 11, status: "missed" },
    { day: 12, status: "played" },
    { day: 13, status: "missed" },
    { day: 14, status: "played" },
    { day: 15, status: "current" },
    { day: 16, status: "future" },
    { day: 17, status: "future" },
    { day: 18, status: "future" },
    { day: 19, status: "future" },
    { day: 20, status: "future" },
    { day: 21, status: "future" },
    { day: 22, status: "future" },
    { day: 23, status: "future" },
    { day: 24, status: "future" },
    { day: 25, status: "future" },
    { day: 26, status: "future" },
    { day: 27, status: "future" },
    { day: 28, status: "future" },
    { day: 29, status: "future" },
    { day: 30, status: "future" },
    { day: 31, status: "future" },
  ];

  $: selectedCalendarDay = 15 + dailyOffset;
  $: dailyLabel = dailyOffset === 0 ? "Today" : `${calendarMonth.split(" ")[0]} ${selectedCalendarDay}`;
  $: selectedValue =
    state?.cells.flat().find((cell) => cell.selected && cell.value !== null)?.value ?? null;
  $: practiceElapsedLabel = formatElapsed(elapsedSeconds);
  $: liveMistakes = validationMode === "live";
  init(wasmUrl)
    .then(() => {
      game = new Game();
      game.start(difficulty, validationMode);
      refresh();
      ready = true;
    })
    .catch((error: unknown) => {
      startupError = error instanceof Error ? error.message : String(error);
    });
  setInterval(() => {
    if (!practiceTimerRunning) return;
    elapsedSeconds = Math.max(0, Math.floor((Date.now() - practiceStartedAt) / 1000));
  }, 1000);

  function refresh() {
    if (!game) return;
    state = game.state() as GameView;
  }

  function start() {
    homeMode = false;
    game?.start(difficulty, validationMode);
    refresh();
  }

  function startPractice() {
    game?.start(difficulty, validationMode);
    practiceStartedAt = Date.now();
    practiceTimerRunning = false;
    elapsedSeconds = 0;
    pendingDifficulty = "";
    syncUrlState();
    refresh();
  }

  function requestPracticeDifficulty(option: string) {
    if (option === difficulty) return;
    pendingDifficulty = option;
  }

  function confirmPracticeDifficulty() {
    if (!pendingDifficulty) return;
    difficulty = pendingDifficulty;
    startPractice();
  }

  function cancelPracticeDifficulty() {
    pendingDifficulty = "";
  }

  function toggleLiveMistakes() {
    validationMode = validationMode === "live" ? "manual" : "live";
    syncUrlState();
  }

  function startDaily() {
    difficulty = "medium";
    validationMode = "manual";
    start();
  }

  function selectCell(row: number, col: number, event: MouseEvent) {
    startPracticeTimer();
    game?.select_cell(row, col, event.shiftKey, event.metaKey || event.ctrlKey);
    refresh();
  }

  function placeDigit(value: number) {
    startPracticeTimer();
    game?.place_digit(value);
    refresh();
  }

  function clearSelection() {
    startPracticeTimer();
    game?.clear_selection();
    refresh();
  }

  function toggleNotes() {
    startPracticeTimer();
    game?.toggle_notes();
    refresh();
  }

  function reset() {
    game?.reset();
    practiceStartedAt = Date.now();
    practiceTimerRunning = false;
    elapsedSeconds = 0;
    confirmResetOpen = false;
    refresh();
  }

  function requestReset() {
    confirmResetOpen = true;
  }

  function cancelReset() {
    confirmResetOpen = false;
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

  function optionLabel(option: string) {
    return option.charAt(0).toUpperCase() + option.slice(1);
  }

  function navigate(route: AppRoute) {
    homeTab = route;
    const path = pathForRoute(route);
    const nextUrl = route === "practice" ? practiceUrl(path) : path;
    if (`${window.location.pathname}${window.location.search}` !== nextUrl) {
      window.history.pushState({}, "", nextUrl);
    }
  }

  function selectDaily(day: number) {
    dailyOffset = day - 15;
  }

  function handleScroll() {
    navScrolled = window.scrollY > 32;
  }

  function handlePopState() {
    homeTab = routeFromPath(window.location.pathname);
    const params = new URLSearchParams(window.location.search);
    difficulty = parseDifficulty(params.get("difficulty"));
    validationMode = params.get("liveMistakes") === "1" ? "live" : "manual";
  }

  function startPracticeTimer() {
    if (homeTab !== "practice" || practiceTimerRunning) return;
    practiceStartedAt = Date.now();
    elapsedSeconds = 0;
    practiceTimerRunning = true;
  }

  function formatElapsed(totalSeconds: number) {
    const minutes = Math.floor(totalSeconds / 60)
      .toString()
      .padStart(2, "0");
    const seconds = (totalSeconds % 60).toString().padStart(2, "0");
    return `${minutes}:${seconds}`;
  }

  function parseDifficulty(value: string | null) {
    return value === "medium" || value === "hard" ? value : "easy";
  }

  function practiceUrl(path = "/practice") {
    const params = new URLSearchParams();
    params.set("difficulty", difficulty);
    if (validationMode === "live") params.set("liveMistakes", "1");
    const query = params.toString();
    return query ? `${path}?${query}` : path;
  }

  function syncUrlState() {
    if (homeTab !== "practice") return;
    window.history.replaceState({}, "", practiceUrl());
  }
</script>

<svelte:window on:keydown={handleKeydown} on:scroll={handleScroll} on:popstate={handlePopState} />

{#if startupError}
  <main class="app loading">
    <section class="startup-error">
      <h1>Sudoku failed to start</h1>
      <p>{startupError}</p>
    </section>
  </main>
{:else if !ready || !state}
  <main class="app loading">Loading</main>
{:else if homeMode}
  <main class="app setup-shell setup-zen">
    <header class="home-nav" class:scrolled={navScrolled}>
      <nav aria-label="Home">
        {#each navRoutes as route}
          <button class:active={homeTab === route.id} on:click={() => navigate(route.id)}>
            {route.label}
          </button>
        {/each}
      </nav>

      <button
        class="account-button"
        class:active={homeTab === "profile"}
        aria-label="Open profile"
        on:click={() => navigate("profile")}
      >
        <span class="avatar">{userProfile.initials}</span>
        <span class="profile-copy">
          <strong>{userProfile.name}</strong>
          <small>{userProfile.handle}</small>
        </span>
        <span class="profile-streak-pill">
          {@html streakIcon}
          {userProfile.streak}
        </span>
      </button>
    </header>

    <section class="setup-panel">
      <Routes
        route={homeTab}
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
        {userProfile}
        {difficulty}
        {liveMistakes}
        {practiceElapsedLabel}
        {pendingDifficulty}
        {confirmResetOpen}
        {optionLabel}
        {selectCell}
        {selectDaily}
        {clearSelection}
        {toggleNotes}
        {placeDigit}
        {submit}
        {requestPracticeDifficulty}
        {confirmPracticeDifficulty}
        {cancelPracticeDifficulty}
        {requestReset}
        {reset}
        {cancelReset}
        {toggleLiveMistakes}
      />
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

        <SudokuSidebarControls
          canClear={state.canClear}
          inputMode={state.inputMode}
          showReset
          onReset={reset}
          onClear={clearSelection}
          onToggleNotes={toggleNotes}
          onPlaceDigit={placeDigit}
        />

        <div class="spacer"></div>
        <button class="primary" on:click={newGame}>New game</button>
      {/if}
    </aside>
  </main>
{/if}
