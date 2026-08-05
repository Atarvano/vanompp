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
  $: currentUrl = currentProject ? `http://localhost:${apachePort}/${currentProject.name}` : ''
  $: hasConn = !!currentProject?.has_conn
  $: db_exists = !!currentProject?.db_exists
  $: db_name = currentProject?.db_name ?? ''
  $: showCreateDb = hasConn && !db_exists
  $: has_conn = hasConn

  function copyUrl() {
    if (!currentUrl) return
    navigator.clipboard.writeText(currentUrl).then(() => {
      copied = true
      dispatch('toast', { msg: `URL dicopy: ${currentUrl}` })
      setTimeout(() => (copied = false), 2000)
    }).catch(() => {
      dispatch('toast', { msg: 'Gagal copy URL' })
    })
  }

  async function openBrowser() {
    openErr = null
    if (!currentUrl) return
    try {
      await openExternal(currentUrl)
    } catch (e) {
      openErr = `Gagal buka browser: ${String(e)}`
    }
  }

  async function openFolder() {
    if (!currentProject) return
    openErr = null
    folderLoading = true
    try {
      await invoke('open_project_folder', { name: currentProject.name })
      dispatch('toast', { msg: `Folder ${currentProject.name} dibuka` })
    } catch (e) {
      const raw = typeof e === 'string' ? e : (e as any)?.toString() || 'Gagal buka folder'
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
  <div class="space-y-4">
    <div class="text-[10px] font-mono uppercase tracking-[0.14em] text-zinc-500">Select Project</div>
    <select
      class="h-10 w-full rounded-lg border border-zinc-200 bg-white px-3 text-sm text-zinc-900 outline-none focus:border-zinc-300 focus:ring-2 focus:ring-[#E9FF70]/50"
      bind:value={$selected}
    >
      <option value="">— pilih project —</option>
      {#each projectList as p}
        <option value={p.name}>{p.name}</option>
      {/each}
    </select>
    {#if currentProject}
      <button
        type="button"
        class="w-full rounded-[16px] border border-amber-100 bg-[#FEFCE8] p-6 md:p-8 text-center cursor-pointer hover:bg-[#fef9c3]/60 transition-colors"
        on:click={copyUrl}
        title="Klik untuk copy URL"
      >
        <span class="font-mono text-[18px] md:text-[20px] font-bold tracking-tight text-zinc-900 break-all block">
          {currentUrl}
        </span>
      </button>
      <div class="flex items-center justify-between gap-2 flex-wrap">
        <button
          class="inline-flex items-center gap-1.5 text-[12px] text-zinc-600 hover:text-zinc-900 transition-colors"
          on:click={openFolder}
          disabled={folderLoading}
        >
          📂 {folderLoading ? 'Buka...' : 'Buka Folder'}
        </button>
        <div class="flex items-center gap-2">
          <button
            class="inline-flex items-center gap-1 rounded-full border border-zinc-200 bg-white px-3.5 py-1.5 text-[12px] font-medium text-zinc-700 hover:bg-zinc-50 transition-colors disabled:opacity-50"
            on:click={openBrowser}
          >
            Buka ↗
          </button>
          <button
            class="inline-flex items-center gap-1 rounded-full bg-black px-3.5 py-1.5 text-[12px] font-medium text-white hover:bg-zinc-800 active:scale-[0.98] transition-all disabled:opacity-50"
            on:click={copyUrl}
          >
            {copied ? 'Copied!' : 'Copy URL'}
          </button>
        </div>
      </div>

      {#if has_conn}
        <div class="flex items-center gap-2 flex-wrap pt-2 border-t border-zinc-100">
          <button
            class="inline-flex items-center gap-1 rounded-full border border-zinc-200 bg-white px-3.5 py-1.5 text-[12px] text-zinc-700 hover:bg-zinc-50 transition-colors"
            on:click={openPhpMyAdmin}
          >
            phpMyAdmin ↗
          </button>
          {#if showCreateDb}
            <button
              on:click={handleCreateDb}
              disabled={dbLoading}
              class="inline-flex items-center gap-1.5 rounded-full bg-amber-100 px-3.5 py-1.5 text-[12px] font-medium text-zinc-900 hover:bg-amber-200 transition-colors disabled:opacity-60"
            >
              {#if dbLoading}
                <span class="h-3 w-3 animate-spin rounded-full border-2 border-zinc-400 border-t-transparent"></span> Bikin DB {db_name || (currentProject?.name.replace(/-/g, '_'))}...
              {:else}
                + Create DB {db_name || (currentProject?.name.replace(/-/g, '_'))}
              {/if}
            </button>
          {:else if db_exists}
            <span class="inline-flex items-center gap-1 rounded-full bg-emerald-50 border border-emerald-200 px-2.5 py-0.5 text-[10px] font-semibold text-emerald-700">DB {db_name} ✓</span>
          {/if}
        </div>
      {/if}

      {#if openErr}
        <p class="text-[11px] font-mono text-red-600">{openErr}</p>
      {/if}
      {#if dbMsg}
        <p class="text-[11px] font-mono text-emerald-700">{dbMsg}</p>
      {/if}
      {#if dbErr}
        <p class="text-[11px] font-mono text-red-600">{dbErr}</p>
      {/if}
    {/if}
  </div>
</BezelCard>
