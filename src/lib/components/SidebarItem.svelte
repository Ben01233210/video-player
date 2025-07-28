<script lang="ts">
  export let label: string;
  export let selected: string;
  export let onClick: () => void;

  $: isActive = selected === label;
</script>

<button
  class={`w-full text-left px-6 py-3 text-sm font-medium transition-colors ${
    isActive ? 'text-blue-600 bg-blue-50' : 'text-gray-700 hover:text-gray-900'
  }`}
  on:click={onClick}
>
  {label}
</button>

