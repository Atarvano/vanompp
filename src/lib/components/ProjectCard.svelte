<script lang="ts">
  import BezelCard from './BezelCard.svelte'
  export let projects: {name:string; path?:string; url?:string}[] = []
  export let selected = ''
  export let port = 8080
  $: currentUrl = selected ? `http://localhost:${port}/${selected}` : ''
  let copied = false
  async function copyUrl(){
    if (!currentUrl) return
    await navigator.clipboard.writeText(currentUrl)
    copied = true
    setTimeout(()=>copied=false, 1800)
  }
  function openBrowser(){
    if (currentUrl) console.log('open', currentUrl)
    if (currentUrl) window.open(currentUrl, '_blank')
  }
</script>
<BezelCard highlight={!!selected}>
  <div class="flex justify-between items-start gap-4">
    <div class="flex-1 min-w-0">
      <h2 class="text-[11px] font-mono font-semibold tracking-[0.14em] uppercase text-zinc-500 mb-3">Project • BIG URL anti-bingung</h2>
      <div class="flex items-center gap-2 mb-4">
        <select bind:value={selected} class="bg-zinc-800 ring-1 ring-white/10 rounded-full px-4 py-2 text-sm text-zinc-200 focus:outline-none focus:ring-volt/50">
          <option value="" disabled>Pilih project...</option>
          {#each projects as p}<option value={p.name}>{p.name} {p.path ? '✓' : ''}</option>{/each}
        </select>
        {#if selected}<span class="inline-flex w-5 h-5 items-center justify-center rounded-full bg-volt text-black text-[10px] font-bold">✓</span>{/if}
      </div>
      {#if currentUrl}
        <div class="group">
          <a href={currentUrl} target="_blank" class="font-mono text-[15px] md:text-[18px] font-medium tracking-tight text-volt underline underline-offset-4 decoration-volt/30 hover:decoration-volt break-all leading-snug">{currentUrl}</a>
          <div class="mt-3 flex gap-2">
            <button on:click={copyUrl} class="rounded-full bg-white/[0.06] ring-1 ring-white/10 px-4 py-2 text-xs font-mono text-zinc-300 hover:bg-white/[0.10] active:scale-[0.98] transition-all flex items-center gap-1.5">
              {copied ? 'Copied!' : 'Copy URL'}
              {#if copied}<span class="w-2 h-2 bg-volt rounded-full"></span>{/if}
            </button>
            <button on:click={openBrowser} class="rounded-full bg-volt text-black px-4 py-2 text-xs font-semibold flex items-center gap-1.5 active:scale-[0.98] transition-transform">Buka di Browser <span class="w-5 h-5 bg-black/10 rounded-full flex items-center justify-center text-[10px]">↗</span></button>
          </div>
        </div>
      {:else}
        <p class="font-mono text-xs text-zinc-600">Pilih project dulu biar URL gede muncul di sini.</p>
      {/if}
    </div>
  </div>
</BezelCard>
