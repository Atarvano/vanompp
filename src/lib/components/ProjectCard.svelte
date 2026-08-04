<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import BezelCard from './BezelCard.svelte'
  import { openExternal } from '$lib/utils/open'
  import { projects, selected, refreshProjects, createDatabase } from '$lib/stores/projects'
  import { services } from '$lib/stores/services'
  import { createEventDispatcher } from 'svelte'
  import { MSG } from '$lib/utils/messages'

  const dispatch = createEventDispatcher()

  export let port = 8080

  let copied = false
  let folderLoading = false
  let openErr: string | null = null
  let dbLoading = false
  let dbMsg: string | null = null
  let dbErr: string | null = null

  $: projectList = $projects
  $: sel = $selected
  $: apachePort = $services.apachePort ?? port
  $: mysqlPort = $services.mysqlPort ?? 3306
  $: currentProject = projectList.find(p => p.name === sel) ?? null
  $: currentUrl = currentProject?.url ?? (sel ? `http://localhost:${apachePort}/${sel}` : '')
  $: hasConn = currentProject?.has_conn ?? false
  $: dbExists = currentProject?.db_exists ?? false
  $: dbName = currentProject?.db_name ?? ''
  $: showCreateDb = hasConn && !dbExists && !!dbName

  async function copyUrl() {
    if (!currentUrl) return
    try {
      await navigator.clipboard.writeText(currentUrl)
    } catch {
      const ta = document.createElement('textarea')
      ta.value = currentUrl
      document.body.appendChild(ta)
      ta.select()
      document.execCommand('copy')
      ta.remove()
    }
    copied = true
    dispatch('toast', { msg: MSG.copyUrlSuccess, kind: 'info' })
    setTimeout(() => (copied = false), 1800)
  }

  async function openBrowser() {
    if (!currentUrl) return
    openErr = null
    await openExternal(currentUrl)
    dispatch('toast', { msg: MSG.openBrowserSuccess(currentUrl), kind: 'info' })
  }

  async function openFolder() {
    if (!sel) return
    folderLoading = true
    openErr = null
    try {
      await invoke('open_project_folder', { name: sel })
    } catch (e) {
      const raw = typeof e === 'string' ? e : String(e)
      openErr = MSG.openFolderFail(raw)
    } finally {
      folderLoading = false
    }
  }

  async function handleCreateDb() {
    if (!currentProject) return
    const targetDb = currentProject.db_name || currentProject.name.replace(/-/g, '_')
    dbLoading = true
    dbMsg = null
    dbErr = null
    try {
      await createDatabase(targetDb, mysqlPort)
      dbMsg = MSG.dbCreated(targetDb)
      await refreshProjects(apachePort)
      dispatch('toast', { msg: dbMsg, kind: 'info' })
    } catch (e) {
      const raw = typeof e === 'string' ? e : (e as any)?.toString() || 'Gagal buat DB'
      if (raw.toLowerCase().includes('mysql belum on') || raw.toLowerCase().includes('connection') || raw.toLowerCase().includes('refused')) {
        dbErr = MSG.mysqlOff
      } else if (raw.toLowerCase().includes('sudah ada') || raw.toLowerCase().includes('exists')) {
        dbMsg = MSG.dbExists(targetDb)
        await refreshProjects(apachePort)
      } else {
        dbErr = MSG.dbFail(raw)
      }
      dispatch('toast', { msg: dbErr ?? dbMsg ?? raw, kind: dbErr ? 'error' as const : 'info' as const })
    } finally {
      dbLoading = false
    }
  }

  async function openPhpMyAdmin() {
    openErr = null
    await openExternal(`http://localhost:${apachePort}/phpmyadmin`)
  }
</script>

<BezelCard highlight={!!sel}>
  <div class="flex justify-between items-start gap-4">
    <div class="flex-1 min-w-0">
      <h2 class="text-[11px] font-mono font-semibold tracking-[0.14em] uppercase text-zinc-500 mb-3">Project • BIG URL anti-bingung</h2>
      <div class="flex items-center gap-2 mb-4 flex-wrap">
        <select bind:value={$selected} class="bg-zinc-800 ring-1 ring-white/10 rounded-full px-4 py-2 text-sm text-zinc-200 focus:outline-none focus:ring-volt/50 min-w-[180px]">
          <option value="" disabled>Pilih project...</option>
          {#each projectList as p}
            <option value={p.name}>{p.name}</option>
          {/each}
        </select>
        {#if sel}<span class="inline-flex w-5 h-5 items-center justify-center rounded-full bg-volt text-black text-[10px] font-bold">✓</span>{/if}
        {#if currentProject}
          <div class="flex gap-1.5 ml-1">
            {#if currentProject.has_index}<span class="text-[10px] font-mono bg-white/[0.06] ring-1 ring-white/10 px-2 py-0.5 rounded-full text-zinc-400">index.php</span>{/if}
            {#if currentProject.has_conn}<span class="text-[10px] font-mono bg-white/[0.06] ring-1 ring-white/10 px-2 py-0.5 rounded-full text-zinc-400">conn.php</span>{/if}
            {#if currentProject.has_gitignore}<span class="text-[10px] font-mono bg-white/[0.06] ring-1 ring-white/10 px-2 py-0.5 rounded-full text-zinc-400">.gitignore</span>{/if}
            {#if currentProject.db_exists}<span class="text-[10px] font-mono bg-volt/20 ring-1 ring-volt/30 px-2 py-0.5 rounded-full text-volt">DB ✓ {currentProject.db_name}</span>{/if}
            {#if showCreateDb}<span class="text-[10px] font-mono bg-amber-500/20 ring-1 ring-amber-500/30 px-2 py-0.5 rounded-full text-amber-300">DB belum ada</span>{/if}
          </div>
        {/if}
      </div>
      {#if currentUrl}
        <div class="group">
          <a href={currentUrl} target="_blank" rel="noopener" class="font-mono text-[15px] md:text-[18px] font-medium tracking-tight text-volt underline underline-offset-4 decoration-volt/30 hover:decoration-volt break-all leading-snug">{currentUrl}</a>
          <div class="mt-3 flex gap-2 flex-wrap">
            <button on:click={copyUrl} class="rounded-full bg-white/[0.06] ring-1 ring-white/10 px-4 py-2 text-xs font-mono text-zinc-300 hover:bg-white/[0.10] active:scale-[0.98] transition-all flex items-center gap-1.5">
              {copied ? 'Copied!' : 'Copy URL'}
              {#if copied}<span class="w-2 h-2 bg-volt rounded-full"></span>{/if}
            </button>
            <button on:click={openBrowser} class="rounded-full bg-volt text-black px-4 py-2 text-xs font-semibold flex items-center gap-1.5 active:scale-[0.98] transition-transform">Buka di Browser <span class="w-5 h-5 bg-black/10 rounded-full flex items-center justify-center text-[10px]">↗</span></button>
            <button on:click={openFolder} disabled={folderLoading} class="rounded-full bg-white/[0.06] ring-1 ring-white/10 px-4 py-2 text-xs font-mono text-zinc-300 hover:bg-white/[0.10] active:scale-[0.98] transition-all flex items-center gap-1.5 disabled:opacity-50">
              {folderLoading ? 'Buka...' : 'Buka Folder'}
            </button>
          </div>
          {#if hasConn}
            <div class="mt-3 flex gap-2 flex-wrap items-center">
              {#if showCreateDb}
                <button on:click={handleCreateDb} disabled={dbLoading} class="rounded-full bg-amber-400 text-black px-4 py-2 text-xs font-semibold flex items-center gap-1.5 active:scale-[0.98] transition-all disabled:opacity-60">
                  {#if dbLoading}
                    <span class="w-3 h-3 border-2 border-black/30 border-t-black rounded-full animate-spin"></span> Bikin DB {dbName}...
                  {:else}
                    [Create DB] {dbName}
                  {/if}
                </button>
              {/if}
              <button on:click={openPhpMyAdmin} class="rounded-full bg-white/[0.06] ring-1 ring-white/10 px-4 py-2 text-xs font-mono text-zinc-300 hover:bg-white/[0.10] active:scale-[0.98] transition-all">
                [Open phpMyAdmin]
              </button>
            </div>
          {/if}
          {#if openErr}<p class="mt-2 text-[11px] font-mono text-red-400">{openErr}</p>{/if}
          {#if dbErr}<p class="mt-2 text-[11px] font-mono text-red-400">{dbErr}</p>{/if}
          {#if dbMsg}<p class="mt-2 text-[11px] font-mono text-volt">{dbMsg}</p>{/if}
        </div>
      {:else}
        <p class="font-mono text-xs text-zinc-600">Pilih project dulu biar URL gede muncul di sini.</p>
      {/if}
    </div>
  </div>
</BezelCard>
