<script>
	import { onMount } from 'svelte';
	import { initializeMpvListener } from '$lib/mpv.svelte';
	import '../app.css';

	onMount(() => {
		initializeMpvListener();
	});
</script>

<main class="m-0 p-0 w-screen h-screen">
<slot />
</main>
	
