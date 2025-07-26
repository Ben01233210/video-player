<script>
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';

  import { page } from '$app/stores';

  let currentTime = 0;
  let duration = 0;

  onMount(() => {
  listen('mpv-event', (event) => {
      const payload = event.payload;
      if (payload.event_type === 'property-change') {
        if (payload.name === 'time-pos') {
          currentTime = payload.data || 0;
        } else if (payload.name === 'duration') {
          duration = payload.data || 0;
        }
      }
    });
  });
</script>

<slot {currentTime} {duration} />
<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: black; /* Clean black background for player */
    color: white;
    font-family: Arial, sans-serif;
  }
</style>
