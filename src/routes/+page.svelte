<script>

    import { goto } from "$app/navigation";

</script>

<div class="font-sans min-h-screen flex flex-col bg-white">
  <main class="flex flex-col lg:flex-row items-center justify-between flex-1 px-8 md:px-16 lg:px-24 xl:px-32 py-8 md:py-12 lg:py-20 gap-8 lg:gap-16">
    <div class="max-w-6xl space-y-10 lg:space-y-20 text-center lg:text-left flex-1">
      <h1 class="font-bold leading-tight tracking-tight text-black text-[clamp(3rem,8vw,7rem)]">
        Revolutionize Your<br />Media Experience
      </h1>
      <p class="text-[clamp(1.25rem,3vw,2rem)] text-gray-600 font-normal max-w-prose mx-auto lg:mx-0 leading-relaxed">
        Experience seamless playback with real‑time subtitles and <br /> translations in 100 languages, powered by advanced
        super <br /> intelligence.
      </p>
      <button 
        class="bg-sky-500 hover:bg-sky-600 text-white  w-xl rounded-lg shadow-lg transition-colors text-[clamp(1.25rem,2.5vw,1.75rem)] py-[clamp(1rem,2.5vw,1.5rem)] px-[clamp(2.5rem,5vw,4rem)] font-semibold"
        onclick={() => goto("/videos")}
      >
        Get Started
      </button>
    </div>

    <div class="relative w-full lg:w-1/2 max-w-[clamp(38rem,85vw,80rem)] mx-auto flex-1">
      <div
        class="absolute inset-0 rounded-full -z-10 scale-110"
        style="background: radial-gradient(circle at center, rgba(0, 162, 255, 0.25) 0%, rgba(0, 162, 255, 0.1) 50%, rgba(0, 162, 255, 0) 70%);"
      ></div>
      <img
        src="/greetings-globe.png"
        alt="Greetings in many languages around a globe"
        class="w-full h-auto drop-shadow-2xl object-cover max-h-[clamp(45rem,70vh,65rem)]"
        loading="eager"
      />
    </div>
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
  }
  
  /* Ensure smooth transitions */
  button {
    transform: translateY(0);
    transition: all 0.2s ease;
  }
  
  button:hover {
    transform: translateY(-1px);
  }
  
  button:active {
    transform: translateY(0);
  }
  
  /* Enhanced image responsiveness */
  img {
    transition: transform 0.3s ease;
  }
  
  img:hover {
    transform: scale(1.02);
  }
</style>

