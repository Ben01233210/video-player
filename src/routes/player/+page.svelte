<script>
	import { onMount, onDestroy } from 'svelte'; // <-- Import onDestroy
	import { page } from '$app/stores';
	import { mpvCommands, playerState, resetPlayerState } from '$lib/mpv.svelte.ts';
	import PlayerControls from '$lib/components/PlayerControls.svelte';
	import { goto } from '$app/navigation';

	onMount(() => {
		const videoPath = $page.url.searchParams.get('video');
		if (videoPath) {
			mpvCommands.play(videoPath);
		}
	});

	// --- THE FIX ---
	// This function is automatically called by Svelte right before the
	// component is removed from the page. It's the perfect place for cleanup.
	function onClick() {
		console.log('Player page unmounting. Stopping video and resetting state.');
    mpvCommands.stop();
		resetPlayerState(); // Clear the UI state for a clean transition
    goto('/')
		
	}
</script>

<!-- The rest of the file remains the same -->
<div class="w-full h-full flex flex-col justify-end">
	<!-- Header with back button and title -->
	<header
		class="fixed top-0 left-0 right-0 p-4 bg-gradient-to-b from-black/70 to-transparent z-10"
	>
		<div class="flex items-center gap-4">
			<button
				onclick={() => onClick()}
				class="bg-white/10 hover:bg-white/20 rounded-full p-2"
				title="Back to list"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-6 w-6"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M15 19l-7-7 7-7"
					/></svg
				>
			</button>
			<h1 class="text-xl font-semibold truncate">{playerState.filename}</h1>
		</div>
	</header>

	<PlayerControls />
</div>