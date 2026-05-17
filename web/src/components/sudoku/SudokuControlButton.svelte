<script lang="ts">
  export let label: string;
  export let active = false;
  export let disabled = false;
  export let icon = "";
  export let className = "";
  export let onPress: () => void = () => {};
</script>

<button
  class={`sudoku-control-button ${className}`}
  class:active
  {disabled}
  aria-label={label}
  title={label}
  onclick={() => onPress()}
>
  {#if icon}
    {@html icon}
  {:else}
    <span>{label}</span>
  {/if}
</button>
