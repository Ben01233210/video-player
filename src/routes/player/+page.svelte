<script>
    import { invoke } from "@tauri-apps/api/core";

  export let currentTime;
  export let duration;

  async function seek(event) {
    const newTime = event.target.value;
    const command = JSON.stringify({ command: ['seek', newTime, 'absolute'] });
    await invoke('send_mpv_command', { commandJson: command });
  }
</script>

<div class="player">
  <!-- MPV renders here automatically via --wid -->
  <div class="overlay">
    <input
      type="range"
      min="0"
      max={duration}
      value={currentTime}
      on:input={seek}
      style="width: 100%;"
    />
    <span>{Math.floor(currentTime)} / {Math.floor(duration)} seconds</span>
  </div>
</div>

<style>
  .player {
    position: relative;
    width: 100vw;
    height: 100vh;
  }
  .overlay {
    position: absolute;
    bottom: 0;
    width: 100%;
    padding: 10px;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
  }
  input[type="range"] {
    flex: 1;
    margin-right: 10px;
  }
</style>
