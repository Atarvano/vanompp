<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import BezelCard from './BezelCard.svelte'
  import { previewSlug, validateSlug } from '$lib/utils/slug'
  import { refreshProjects, selected as selectedStore, projects as projectsStore } from '$lib/stores/projects'
  import { MSG, t } from '$lib/utils/messages'
  $: tt = $t

  const dispatch = createEventDispatcher()

  let name = ''
  let dbChecked = true
  let dbName = ''
  let loading = false

  function sanitizeDbInput(s: string) {
    return s.trim().toLowerCase().replace(/-/g,'_').replace(/[^a-z0-9_]/g,'')
  }
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
        warnMsg = `${MSG.mysqlOff} - DB "${dbFinalSanitized}" belum kekonek, coba Start MySQL dulu.`
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
  <div class="space-y-4">
    <div class="text-[10px] font-mono uppercase tracking-[0.14em] text-zinc-500">{tt('newProjectTitle')}</div>

    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:gap-2">
      <div class="flex flex-1 items-center overflow-hidden rounded-lg border border-zinc-200 bg-white">
        <input
          id="create-input"
          bind:value={name}
          on:keydown={onKeydown}
          placeholder="toko-buku"
          disabled={loading}
          class="h-9 flex-1 border-0 bg-transparent px-3 text-sm text-zinc-900 placeholder:text-zinc-400 outline-none focus:ring-0"
        />
        <span class="whitespace-nowrap px-2 font-mono text-[11px] text-zinc-400">
          -> www/{slug || 'nama'}/
        </span>
      </div>

      <label class="inline-flex items-center gap-1.5 text-[12px] text-zinc-700">
        <input type="checkbox" bind:checked={dbChecked} class="h-4 w-4 rounded border-zinc-300 text-black focus:ring-[#E9FF70] accent-black" />
        {tt('createDbQ')}
      </label>
    </div>

    {#if dbChecked && slug}
      <div class="flex items-center gap-2">
        <input
          bind:value={dbName}
          on:input={(e)=>{ dbName = sanitizeDbInput((e.target as HTMLInputElement).value) }}
          placeholder={dbFinalSanitized || 'nama_db custom'}
          class="h-8 w-[180px] rounded-full border border-zinc-200 bg-white px-3 text-[12px] font-mono text-zinc-900 placeholder:text-zinc-400 outline-none focus:border-zinc-300 focus:ring-1 focus:ring-[#E9FF70]/50"
        />
        <span class="text-[10px] font-mono text-zinc-400">→ {dbFinalSanitized || '...'}</span>
        {#if dbName && dbFinalSanitized && dbFinalSanitized.length > 0 && dbFinalSanitized.length <= 64}
          <span class="text-[10px] text-emerald-700 font-mono">{tt('customOk')}</span>
        {/if}
      </div>
    {/if}

    {#if error}
      <p class="text-[11px] font-mono text-red-600">{error}</p>
    {/if}
    {#if warnMsg}
      <p class="text-[11px] font-mono text-amber-700 bg-amber-50 border border-amber-200 rounded-lg px-3 py-2">{warnMsg}</p>
    {/if}

    <div class="flex justify-end">
      <button
        class="inline-flex items-center gap-2 rounded-full bg-black px-4 py-2 text-[12px] font-medium text-white hover:bg-zinc-800 active:scale-[0.98] transition-all disabled:opacity-50"
        disabled={!canCreate}
        on:click={handleCreate}
      >
        {loading ? '...' : tt('createProject')}
        <span class="flex h-4 w-4 items-center justify-center rounded-full bg-white/20 text-[10px]">+</span>
      </button>
    </div>
  </div>
</BezelCard>
