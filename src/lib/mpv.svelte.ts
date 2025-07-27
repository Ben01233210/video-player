import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow  } from '@tauri-apps/api/window';

// --- Types ---
export interface MpvEvent {
	event_type: string;
	name?: string;
	data?: any;
}

// --- Reactive State Store ($state) ---
export const playerState = $state({
	isPlaying: false,
	currentTime: 0,
	duration: 0,
	filename: '',
	volume: 100 // Default volume
});

// --- Commands ---
export const mpvCommands = {
	play: (path: string) => invoke('play', { path }),
	pause: () => invoke('pause'),
	resume: () => invoke('resume'),
	seek: (timeInSeconds: number) => invoke('seek', { timeInSeconds }),
	setVolume: (volume: number) => invoke('set_volume', { volume }),
	toggleFullscreen: () => invoke('toggle_fullscreen', { window: () => getCurrentWindow() }),
	stop: () => invoke('stop')
};

export function resetPlayerState() {
	playerState.isPlaying = false;
	playerState.currentTime = 0;
	playerState.duration = 0;
	playerState.filename = '';
}

// --- Event Listener ---
export async function initializeMpvListener() {
	await listen<MpvEvent>('mpv-event', (event) => {
		const { event_type, name, data } = event.payload;

		if (event_type === 'property-change') {
			switch (name) {
				case 'time-pos':
					playerState.currentTime = data ?? playerState.currentTime;
					break;
				case 'duration':
					playerState.duration = data ?? playerState.duration;
					break;
				case 'pause':
					playerState.isPlaying = !data;
					break;
				case 'filename':
					playerState.filename = data?.split(/[\\/]/).pop() ?? ''; // Show only filename
					break;
				case 'volume':
					playerState.volume = data ?? playerState.volume;
					break;
			}
		}

		if (event_type === 'eof-reached') {
			playerState.isPlaying = false;
			playerState.currentTime = playerState.duration;
			// Here you could implement auto-play next
		}
	});
}