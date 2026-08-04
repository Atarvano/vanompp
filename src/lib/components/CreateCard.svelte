<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import BezelCard from './BezelCard.svelte'
  import { previewSlug, validateSlug } from '$lib/utils/slug'
  import { refreshProjects, selected as selectedStore } from '$lib/stores/projects'
  import { projects as projectsStore } from '$lib/stores/projects'
  import { MSG } from '$lib/utils/messages'

  const dispatch = createEventDispatcher()

  let name = ''
  let dbChecked = true
  let dbName = ''
  let loading = false
  let errorMsg: string | null = null
  let warnMsg: string | null = null

  $: slug = previewSlug(name)
  $: dbFinal = dbName.trim() || slug
  $: dbFinalSanitized = dbFinal.replace(/-/g, '_').toLowerCase()
  $: validationError = (() => {
    if (!name.trim()) return null
    if (!slug) return MSG.slugEmpty
    return validateSlug(slug)
  })()
  $: error = errorMsg || validationError
  $: canCreate = !!slug && !validationError && !loading

  async function handleCreate() {
    if (!canCreate) return
    loading = true
    errorMsg = null
    warnMsg = null
    try {
      const result = await invoke<any>('create_project', {
        name: slug,
        create_db: dbChecked,
        db_name: dbChecked ? dbFinal : '',
        createDb: dbChecked,
        dbName: dbChecked ? dbFinal : ''
      } as any)
      await refreshProjects()
      const createdName = result.name as string
      const resultHasDb = result.db_exists ?? result.dbExists
      if (dbChecked && resultHasDb === false) {
        warnMsg = MSG.mysqlOff
      }
      let currentList: any[] = []
      const unsub = projectsStore.subscribe((v) => (currentList = v))
      unsub()
      const found = currentList.find((p) => p.name === createdName)
      if (dbChecked && found && !found.db_exists) {
        warnMsg = `${MSG.mysqlOff} — DB "${dbFinalSanitized}" belum kekonek, coba Start MySQL dulu.`
      }

      selectedStore.set(createdName)
      dispatch('created', { name: createdName, db: dbChecked ? dbFinalSanitized : '', warn: warnMsg })
      name = ''
      dbName = ''
    } catch (e) {
      const msg = typeof e === 'string' ? e : (e as any)?.toString() || MSG.unexpectedError
      if (msg.toLowerCase().includes('sudah ada') || msg.toLowerCase().includes('already exists') || msg.toLowerCase().includes('duplicate') || msg.includes('Folder')) {
        errorMsg = MSG.folderExistsSuggest(slug)
      } else if (msg.toLowerCase().includes('invalid') || msg.toLowerCase().includes('ga boleh')) {
        errorMsg = msg
      } else if (msg.includes('MySQL belum ON')) {
        await refreshProjects()
        selectedStore.set(slug)
        warnMsg = `Project "${slug}" dibuat tapi ${msg}`
        dispatch('created', { name: slug, db: dbFinalSanitized, warn: warnMsg })
        name = ''
        dbName = ''
      } else {
        errorMsg = `Gagal: ${msg}`
      }
    } finally {
      loading = false
    }
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && canCreate) handleCreate()
  }
</script>

<BezelCard>
  <h2 class="text-[11px] font-mono font-semibold tracking-[0.14em] uppercase text-zinc-500 mb-4">Buat Project Baru</h2>
  <div class="space-y-3">
    <div>
      <input
        id="create-input"
        bind:value={name}
        on:keydown={onKeydown}
        placeholder="nama-project (contoh: toko-buku)"
        disabled={loading}
        class="w-full bg-zinc-800/70 ring-1 ring-white/10 rounded-[0.85rem] px-4 py-3 text-sm text-white placeholder:text-zinc-600 focus:outline-none focus:ring-volt/50 transition-all disabled:opacity-60"
      />
      <div class="mt-2 flex flex-wrap items-center gap-2 text-[11px] font-mono">
        {#if slug}
          <span class="text-zinc-500">slug →</span><code class="bg-white/[0.06] px-2 py-0.5 rounded-full text-volt">{slug}</code>
        {:else}
          <span class="text-zinc-600">ketik nama dulu — {MSG.invalidSlug}</span>
        {/if}
        {#if error}
          <span class="text-red-400 ml-1">{error}</span>
        {/if}
      </div>
    </div>
    <label class="flex items-center gap-2.5 text-sm text-zinc-300 cursor-pointer select-none">
      <input type="checkbox" bind:checked={dbChecked} disabled={loading} class="w-4 h-4 rounded accent-volt bg-zinc-800 ring-1 ring-white/10 disabled:opacity-50" />
      <span>Buat database MySQL sekalian?</span>
    </label>
    {#if dbChecked}
      <input bind:value={dbName} placeholder="nama DB (default = {slug || 'slug'})" disabled={loading} class="w-full bg-zinc-800/60 ring-1 ring-white/10 rounded-[0.85rem] px-4 py-2.5 text-sm text-white placeholder:text-zinc-600 focus:outline-none focus:ring-volt/30 disabled:opacity-60" />
      <p class="text-[11px] font-mono text-zinc-500">DB: <code class="text-zinc-300">{dbFinalSanitized || '(kosong)'}</code> — pastikan MySQL ON biar langsung kebikin</p>
    {/if}
    {#if warnMsg}
      <p class="text-[11px] font-mono text-amber-300 bg-amber-500/10 ring-1 ring-amber-500/20 rounded-xl px-3 py-2">{warnMsg}</p>
    {/if}
    <button
      on:click={handleCreate}
      disabled={!canCreate}
      class="mt-1 w-full md:w-auto rounded-full {canCreate ? 'bg-white text-black hover:bg-zinc-100' : 'bg-zinc-800 text-zinc-600'} px-6 py-3 text-sm font-semibold flex items-center justify-center gap-2 active:scale-[0.98] transition-all duration-[600ms] ease-[cubic-bezier(0.32,0.72,0,1)] disabled:cursor-not-allowed"
    >
      {#if loading}
        <span class="w-4 h-4 border-2 border-zinc-500 border-t-transparent rounded-full animate-spin"></span>
        Bikin...
      {:else}
        + Buat Project
        <span class="w-7 h-7 {canCreate ? 'bg-black/10' : 'bg-white/10'} rounded-full flex items-center justify-center text-xs">↗</span>
      {/if}
    </button>
  </div>
</BezelCard>
