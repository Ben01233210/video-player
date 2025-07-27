<script>
	import { playerState, mpvCommands } from '$lib/mpv.svelte';
	
	// You should create these simple SVG icon components in a subfolder
	// For this example, I'll use text placeholders, but recommend using real SVGs.
	// e.g. import PlayIcon from './icons/PlayIcon.svelte'

	function formatTime(seconds) {
		if (isNaN(seconds) || seconds < 0) return '00:00';
		const date = new Date(seconds * 1000);
		const hh = date.getUTCHours();
		const mm = date.getUTCMinutes();
		const ss = date.getUTCSeconds().toString().padStart(2, '0');
		return hh ? `${hh}:${mm.toString().padStart(2, '0')}:${ss}` : `${mm}:${ss}`;
	}

	function handleSeek(e) {
		mpvCommands.seek(parseFloat(e.currentTarget.value));
	}

	function handleVolume(e) {
		mpvCommands.setVolume(parseFloat(e.currentTarget.value));
	}
</script>

<div
	class="w-full p-4 bg-gradient-to-t from-black/70 to-transparent flex flex-col gap-2"
>
	<!-- Seek Bar -->
	<input
		type="range"
		min="0"
		max={playerState.duration || 1}
		value={playerState.currentTime}
		oninput={handleSeek}
		class="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer range-lg"
	/>

	<!-- Main Controls -->
	<div class="flex justify-between items-center text-gray-200">
		<!-- Left Side: Play, Volume -->
		<div class="flex items-center gap-4">
			<button
				onclick={playerState.isPlaying ? mpvCommands.pause : mpvCommands.resume}
				class="text-4xl"
			>
				{playerState.isPlaying ? '❚❚' : '▶'}
			</button>

			<div class="flex items-center gap-2 group">
				<button class="text-2xl">🔊</button>
				<input
					type="range"
					min="0"
					max="100"
					value={playerState.volume}
					oninput={handleVolume}
					class="w-24 h-1 bg-gray-600 rounded-lg appearance-none cursor-pointer range-sm"
				/>
			</div>
		</div>

		<!-- Center: Time -->
		<div>
			<span>{formatTime(playerState.currentTime)}</span>
			<span class="mx-1">/</span>
			<span>{formatTime(playerState.duration)}</span>
		</div>

		<!-- Right Side: Fullscreen -->
		<div class="flex items-center gap-4">
			<button onclick={mpvCommands.toggleFullscreen} class="text-2xl">
                ⛶
			</button>
		</div>
	</div>
</div>