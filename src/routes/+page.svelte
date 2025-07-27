<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	let videos = $state([]);
	let isLoading = $state(true);

	onMount(async () => {
		videos = await invoke('get_videos');
		isLoading = false;
	});

	function getFileName(path) {
		return path.split(/[\\/]/).pop();
	}
</script>

<div class=" p-8 w-full h-full  bg-black">
	<h1 class="text-4xl font-bold mb-8 text-white">Your Videos</h1>

	{#if isLoading}
		<p>Scanning for videos...</p>
	{:else if videos.length === 0}
		<p>No videos found in your home directory.</p>
	{:else}
		<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
			{#each videos as video}
				<a
					href={`/player?video=${encodeURIComponent(video)}`}
					class="bg-gray-800 rounded-lg shadow-lg overflow-hidden transition-transform hover:scale-105"
				>
					<div class="h-32 bg-gray-700 flex items-center justify-center">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 text-gray-500" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" /></svg>
					</div>
					<div class="p-4">
						<h3 class="font-semibold truncate" title={getFileName(video)}>
							{getFileName(video)}
						</h3>
					</div>
				</a>
			{/each}
		</div>
	{/if}
</div>