<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import {
    Search,
    Heart,
    Folder,
    Plus,
    Play,
    ArrowLeft,
    Settings,
    RotateCcw,
    HelpCircle,
    Maximize2
  } from '@lucide/svelte';

  import SidebarItem from '$lib/components/SidebarItem.svelte';

  let videos: string[] = [];
  let isLoading = true;
  let searchQuery = '';
  let activeSection: 'Video on my PC' | 'History' | 'Favorites' = 'Video on my PC';

  onMount(async () => {
    videos = await invoke<string[]>('get_videos');
    isLoading = false;
  });

  const getFileName = (path: string) => path.split(/[/\\]/).pop() ?? path;
  const formatDuration = (duration = '10:45') => duration;
  const handleVideoClick = (video: string) => {
    window.location.href = `/player?video=${encodeURIComponent(video)}`;
  };

  $: filteredVideos =
    searchQuery.trim().length === 0
      ? videos
      : videos.filter((v) => getFileName(v).toLowerCase().includes(searchQuery.toLowerCase()));
</script>

<div class="flex flex-col h-screen bg-gray-100 font-sans text-gray-800 select-none">
  <div class="flex items-center justify-between px-4 py-2 border-b border-gray-200 bg-white shadow-sm">
    <div class="flex items-center space-x-4">
      <div class="flex items-center space-x-1.5">
        <div class="w-3 h-3 rounded-full" style="background:#ff5f56"></div>
        <div class="w-3 h-3 rounded-full" style="background:#ffbd2e"></div>
        <div class="w-3 h-3 rounded-full" style="background:#27c93f"></div>
      </div>

      <span class="text-sm text-gray-700 bg-gray-100 border border-gray-300 rounded px-2 py-0.5">
        Mode: Entertainment
      </span>

      <HelpCircle class="w-4 h-4 text-gray-500" />
      <Maximize2 class="w-4 h-4 text-gray-500" />
    </div>

    <div class="flex items-center space-x-3">
      <button
        class="px-4 py-1.5 text-sm font-medium text-white bg-blue-600 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        My media
      </button>

      <button class="flex items-center space-x-1 text-sm text-gray-700 hover:text-gray-900">
        <RotateCcw class="w-4 h-4" />
        <span>Updates</span>
      </button>

      <button class="flex items-center space-x-1 text-sm text-gray-700 hover:text-gray-900">
        <Settings class="w-4 h-4" />
        <span>Settings</span>
      </button>
    </div>
  </div>

  <div class="flex flex-1 overflow-hidden">
    <aside class="w-64 bg-white border-r border-gray-200">
      <nav class="mt-6 space-y-1">
        <SidebarItem label="Video on my PC" selected={activeSection} onClick={() => (activeSection = 'Video on my PC')} />
        <SidebarItem label="History" selected={activeSection} onClick={() => (activeSection = 'History')} />
        <SidebarItem label="Favorites" selected={activeSection} onClick={() => (activeSection = 'Favorites')} />
      </nav>
    </aside>

    <section class="flex flex-col flex-1 bg-white">
      <header class="flex items-center justify-between px-6 py-4 border-b border-gray-200">
        <button class="flex items-center space-x-2 text-sm text-gray-600 hover:text-gray-800">
          <ArrowLeft class="w-4 h-4" />
          <span>Back to Player</span>
        </button>

        <h1 class="text-xl font-semibold text-gray-900">Video on my PC</h1>

        <button class="flex items-center space-x-2 text-sm text-gray-600 hover:text-gray-800">
          <Plus class="w-4 h-4" />
          <span>Add selected videos to playlist</span>
        </button>
      </header>

      <div class="flex flex-col flex-1 p-6 overflow-auto">
        <div class="relative w-full mb-4">
          <Search class="absolute w-4 h-4 text-gray-400 left-3 top-1/2 -translate-y-1/2 pointer-events-none" />
          <input
            type="text"
            placeholder="Search"
            class="w-full py-2 pl-10 pr-4 text-sm bg-gray-50 border border-gray-200 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            bind:value={searchQuery}
          />
        </div>

        <div class="mb-4 text-sm text-gray-600">{filteredVideos.length} items found</div>

        {#if isLoading}
          <div class="flex items-center justify-center flex-1 py-12 text-gray-500">Scanning for videos…</div>
        {:else if filteredVideos.length === 0}
          <div class="flex items-center justify-center flex-1 py-12 text-gray-500">No videos found in your home directory.</div>
        {:else}
          <div class="divide-y divide-gray-100">
            {#each filteredVideos as video}
              <div
                class="flex items-center px-4 py-3 cursor-pointer hover:bg-gray-50 group"
                role="button"
                tabindex="0"
                on:click={() => handleVideoClick(video)}
                on:keydown={(e) => e.key === 'Enter' && handleVideoClick(video)}
              >
                <Play class="w-4 h-4 text-gray-600 mr-4 flex-none fill-current group-hover:text-blue-600" />

                <div class="flex-1 min-w-0 mr-4 truncate text-sm text-gray-900" title={getFileName(video)}>
                  {getFileName(video)}
                </div>

                <div class="mr-6 text-sm text-gray-500 flex-none">{formatDuration()}</div>

                <div class="flex items-center space-x-3 flex-none">
                  <button class="text-gray-400 hover:text-red-500" title="Favorite">
                    <Heart class="w-4 h-4" />
                  </button>
                  <button class="text-gray-400 hover:text-blue-500" title="Folder">
                    <Folder class="w-4 h-4" />
                  </button>
                  <button class="text-gray-400 hover:text-green-600" title="Add">
                    <Plus class="w-4 h-4" />
                  </button>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </section>
  </div>
</div>

<style>
  ::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }
  ::-webkit-scrollbar-track {
    background: transparent;
  }
  ::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.1);
    border-radius: 4px;
  }
  ::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.2);
  }
</style>

