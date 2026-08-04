<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import BezelCard from './BezelCard.svelte'
  import { slugifyInput, validateSlug } from '$lib/utils/slug'
  const dispatch = createEventDispatcher()
  let name = ''
  let dbChecked = true
  let dbName = ''
  $: slug = slugifyInput(name)
  $: dbFinal = dbName || slug
  $: error = validateSlug(slug)
  $: canCreate = !!slug && !error
  function handleCreate(){
    if (!canCreate) return
    dispatch('created', { name: slug, db: dbChecked ? dbFinal : '' })
    name = ''; dbName = ''
  }
</script>
<BezelCard>
  <h2 class="text-[11px] font-mono font-semibold tracking-[0.14em] uppercase text-zinc-500 mb-4">Buat Project Baru</h2>
  <div class="space-y-3">
    <div>
      <input id="create-input" bind:value={name} placeholder="nama-project (contoh: toko-buku)" class="w-full bg-zinc-800/70 ring-1 ring-white/10 rounded-[0.85rem] px-4 py-3 text-sm text-white placeholder:text-zinc-600 focus:outline-none focus:ring-volt/50 transition-all" />
      <div class="mt-2 flex items-center gap-2 text-[11px] font-mono">
        {#if slug}<span class="text-zinc-500">slug →</span><code class="bg-white/[0.06] px-2 py-0.5 rounded-full text-volt">{slug}</code>{:else}<span class="text-zinc-600">ketik nama dulu</span>{/if}
        {#if error}<span class="text-red-400 ml-2">{error}</span>{/if}
      </div>
    </div>
    <label class="flex items-center gap-2.5 text-sm text-zinc-300 cursor-pointer select-none">
      <input type="checkbox" bind:checked={dbChecked} class="w-4 h-4 rounded accent-volt bg-zinc-800 ring-1 ring-white/10" />
      <span>[x] Buat database MySQL sekalian?</span>
    </label>
    {#if dbChecked}
      <input bind:value={dbName} placeholder="nama DB (default = {slug || 'slug'})" class="w-full bg-zinc-800/60 ring-1 ring-white/10 rounded-[0.85rem] px-4 py-2.5 text-sm text-white placeholder:text-zinc-600 focus:outline-none focus:ring-volt/30" />
    {/if}
    <button on:click={handleCreate} disabled={!canCreate} class="mt-1 w-full md:w-auto rounded-full {canCreate ? 'bg-white text-black hover:bg-zinc-100' : 'bg-zinc-800 text-zinc-600'} px-6 py-3 text-sm font-semibold flex items-center justify-center gap-2 active:scale-[0.98] transition-all duration-[600ms] ease-[cubic-bezier(0.32,0.72,0,1)] disabled:cursor-not-allowed">
      + Buat Project
      <span class="w-7 h-7 {canCreate ? 'bg-black/10' : 'bg-white/10'} rounded-full flex items-center justify-center text-xs">↗</span>
    </button>
  </div>
</BezelCard>
