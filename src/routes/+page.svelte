<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
    import { invoke } from '@tauri-apps/api/core';

  let videos = $state([]);

  onMount(async () => {
    videos = await invoke('get_videos');
  });

  async function playVideo(path) {
    const command = JSON.stringify({ command: ['loadfile', path] });

    await invoke('send_mpv_command', { commandJson: command });
    goto('/player');
  }
</script>

<div class="list">
  <h1>Your Videos</h1>
  <ul>
    {#each videos as video}
      <button on:click={() => playVideo(video)}>{video}</button>
    {/each}
  </ul>
</div>

<style>
  .list {
    padding: 20px;
    max-height: 100vh;
    overflow-y: auto;
  }
  ul {
    list-style: none;
    padding: 0;
  }
  li {
    cursor: pointer;
    padding: 10px;
    border-bottom: 1px solid #ccc;
  }
  li:hover {
    background: #333;
  }
</style>
