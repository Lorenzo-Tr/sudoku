<script lang="ts">
  import "./leaderboard.css";

  export let leaderboard: Array<{ rank: number; name: string; time: string; score: string }> = [];
</script>

<section class="leaderboard-view">
  <div>
    <span class="daily-kicker">Today&apos;s challenge</span>
    <h1>Leaderboard</h1>
  </div>
  <div class="leaderboard-list">
    {#each leaderboard as player}
      <div class:you={player.name === "You"} class="leader-row large">
        <span>{player.rank}</span>
        <strong>{player.name}</strong>
        <em>{player.time}</em>
        <small>{player.score}</small>
      </div>
    {/each}
  </div>
</section>
