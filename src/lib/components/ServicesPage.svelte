<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import BezelCard from './BezelCard.svelte'
  import TerminalTab from './TerminalTab.svelte'
  import {
    services,
    loading,
    lastError,
    startService,
    stopService,
    startAllServices,
    stopAllServices,
    checkPorts,
    toConflicts,
    repairMysql,
    DEFAULT_APACHE_PORT,
    DEFAULT_MYSQL_PORT,
    isCustomPort,
    resetPersistedPort,
    type ConflictInfo
  } from '$lib/stores/services'
  import { t } from '$lib/utils/messages'
  import { MSG } from '$lib/utils/messages'

  const dispatch = createEventDispatcher<{
    conflict: { conflicts: ConflictInfo[]; error?: string }
    error: { msg: string }
    toast: { msg: string }
    openLogs: { service: 'apache' | 'mysql' | 'php' }
  }>()

  $: tt = $t
  type Tab = 'apache' | 'mysql' | 'terminal'
  let tab: Tab = 'apache'

  $: svc = $services
  $: ld = $loading
  $: err = $lastError
  $: allRunning = svc.apache && svc.mysql
  $: anyRunning = svc.apache || svc.mysql

  async function toggle(name: 'apache' | 'mysql') {
    if (ld[name] || ld.all) return
    if (svc[name]) {
      try {
        const msg = await stopService(name)
        dispatch('toast', { msg: `${name.toUpperCase()} dimatiin — ${msg}` })
      } catch (e) {
        dispatch('error', { msg: typeof e === 'string' ? e : String(e) })
      }
      return
    }
    try {
      const infos = await checkPorts()
      const relevantPort = name === 'apache' ? svc.apachePort : svc.mysqlPort
      const pi = infos.find((p) => p.port === relevantPort)
      if (pi && !pi.free) {
        dispatch('conflict', {
          conflicts: [{ name, port: pi.port, suggest: pi.suggest }],
          error: name === 'apache' ? MSG.apachePortConflict(pi.port, pi.suggest) : MSG.mysqlPortConflict(pi.port, pi.suggest)
        })
        return
      }
    } catch {}
    try {
      const portToUse = name === 'apache' ? svc.apachePort : svc.mysqlPort
      const msg = await startService(name, portToUse)
      dispatch('toast', { msg: `${name.toUpperCase()} nyala — ${msg}` })
    } catch (e) {
      const msg = typeof e === 'string' ? e : String(e)
      const lower = msg.toLowerCase()
      if (lower.includes('port') || lower.includes('dipakai') || lower.includes('kepake') || lower.includes('already') || lower.includes('in use') || lower.includes('conflict')) {
        try {
          const infos = await checkPorts()
          const conflicts = toConflicts(infos)
          if (conflicts.length) {
            dispatch('conflict', { conflicts, error: msg })
            return
          }
        } catch {}
      }
      dispatch('error', { msg })
    }
  }

  async function handleStartAll() {
    if (ld.all || allRunning) return
    try {
      const infos = await checkPorts()
      const conflicts = toConflicts(infos)
      if (conflicts.length) {
        dispatch('conflict', { conflicts, error: MSG.portKepake(conflicts.map(c=>c.port)) })
        return
      }
    } catch {}
    try {
      const msgs = await startAllServices()
      dispatch('toast', { msg: msgs.join(' • ') })
    } catch (e) {
      const msg = typeof e === 'string' ? e : String(e)
      if (msg.toLowerCase().includes('port') || msg.toLowerCase().includes('dipakai') || msg.toLowerCase().includes('kepake') || msg.toLowerCase().includes('in use')) {
        try {
          const infos = await checkPorts()
          const conflicts = toConflicts(infos)
          if (conflicts.length) {
            dispatch('conflict', { conflicts, error: msg })
            return
          }
        } catch {}
      }
      dispatch('error', { msg })
    }
  }

  async function handleStopAll() {
    if (ld.all) return
    try {
      const msgs = await stopAllServices()
      dispatch('toast', { msg: msgs.join(' • ') })
    } catch (e) {
      dispatch('error', { msg: typeof e === 'string' ? e : String(e) })
    }
  }

  async function handleRepairMysql() {
    try {
      const msg = await repairMysql()
      dispatch('toast', { msg })
    } catch (e) {
      dispatch('error', { msg: typeof e === 'string' ? e : String(e) })
    }
  }

  async function handleResetPort(name: 'apache' | 'mysql') {
    try {
      await resetPersistedPort(name)
      dispatch('toast', { msg: `${name.toUpperCase()} port direset ke default ${name==='apache'?DEFAULT_APACHE_PORT:DEFAULT_MYSQL_PORT}` })
    } catch (e) {
      dispatch('error', { msg: `Gagal reset ${name}: ${String(e)}` })
    }
  }
</script>

<BezelCard highlight={anyRunning}>
  <div class="flex items-center justify-between">
    <h2 class="text-[11px] font-mono font-semibold tracking-[0.14em] uppercase text-zinc-500">Services</h2>
    {#if $loading.all}
      <span class="inline-flex items-center gap-1 text-[10px] text-zinc-400">
        <span class="w-3 h-3 border border-zinc-400 border-t-transparent rounded-full animate-spin"></span>
        proses...
      </span>
    {/if}
  </div>

  <!-- Tabs -->
  <div class="mt-3 flex gap-1 rounded-full bg-zinc-100 p-1 w-fit">
    <button
      class="rounded-full px-3.5 py-1.5 text-[12px] font-medium transition-colors {tab==='apache' ? 'bg-zinc-900 text-white' : 'text-zinc-600 hover:text-zinc-900'}"
      on:click={() => (tab = 'apache')}
    >Apache</button>
    <button
      class="rounded-full px-3.5 py-1.5 text-[12px] font-medium transition-colors {tab==='mysql' ? 'bg-zinc-900 text-white' : 'text-zinc-600 hover:text-zinc-900'}"
      on:click={() => (tab = 'mysql')}
    >MySQL</button>
    <button
      class="rounded-full px-3.5 py-1.5 text-[12px] font-medium transition-colors {tab==='terminal' ? 'bg-zinc-900 text-white' : 'text-zinc-600 hover:text-zinc-900'}"
      on:click={() => (tab = 'terminal')}
    >Terminal</button>
  </div>

  {#if tab === 'apache'}
    <div class="mt-4 flex items-center justify-between py-3">
      <div class="flex-1 min-w-0">
        <div class="text-[10px] font-mono uppercase tracking-[0.14em] text-zinc-500">Apache</div>
        <div class="mt-1 flex items-center gap-2 flex-wrap">
          {#if $services.apache}
            <span class="inline-flex items-center gap-1 rounded-full bg-[#E9FF70] px-2.5 py-0.5 text-[10px] font-bold text-black"><span class="h-1.5 w-1.5 rounded-full bg-black"></span> ON</span>
          {:else}
            <span class="inline-flex items-center rounded-full bg-zinc-100 px-2.5 py-0.5 text-[10px] font-semibold text-zinc-500">OFF</span>
          {/if}
          <span class="text-[11px] font-mono text-zinc-600">Port: {$services.apachePort}</span>
          {#if isCustomPort('apache', $services.apachePort)}
            <span class="text-[10px] font-mono text-zinc-400">custom</span>
            <button class="text-[10px] text-zinc-500 underline hover:text-zinc-800" on:click={() => handleResetPort('apache')} title="Reset ke 8080">×</button>
          {/if}
          <button class="rounded-full border border-zinc-200 bg-white px-2.5 py-0.5 text-[10px] text-zinc-600 hover:bg-zinc-50" on:click={() => dispatch('openLogs', { service: 'apache' })}>Logs</button>
        </div>
      </div>
      <div>
        {#if $services.apache}
          <button class="text-[12px] text-zinc-700 hover:text-black disabled:opacity-40" disabled={$loading.apache} on:click={() => toggle('apache')}>{$loading.apache ? '...' : 'Stop'}</button>
        {:else}
          <button class="inline-flex items-center gap-1 rounded-full bg-black px-3.5 py-1.5 text-[12px] font-medium text-white hover:bg-zinc-800 disabled:opacity-50" disabled={$loading.apache} on:click={() => toggle('apache')}>{$loading.apache ? '...' : 'Start'} <span class="text-[10px]">↗</span></button>
        {/if}
      </div>
    </div>
  {:else if tab === 'mysql'}
    <div class="mt-4 flex items-center justify-between py-3">
      <div class="flex-1 min-w-0">
        <div class="text-[10px] font-mono uppercase tracking-[0.14em] text-zinc-500">MySQL</div>
        <div class="mt-1 flex items-center gap-2 flex-wrap">
          {#if $services.mysql}
            <span class="inline-flex items-center gap-1 rounded-full bg-[#E9FF70] px-2.5 py-0.5 text-[10px] font-bold text-black"><span class="h-1.5 w-1.5 rounded-full bg-black"></span> ON</span>
          {:else}
            <span class="inline-flex items-center rounded-full bg-zinc-100 px-2.5 py-0.5 text-[10px] font-semibold text-zinc-500">OFF</span>
          {/if}
          <span class="text-[11px] font-mono text-zinc-600">Port: {$services.mysqlPort}</span>
          {#if isCustomPort('mysql', $services.mysqlPort)}
            <span class="text-[10px] font-mono text-zinc-400">custom</span>
            <button class="text-[10px] text-zinc-500 underline hover:text-zinc-800" on:click={() => handleResetPort('mysql')} title="Reset ke 3306">×</button>
          {/if}
          <button class="rounded-full border border-zinc-200 bg-white px-2.5 py-0.5 text-[10px] text-zinc-600 hover:bg-zinc-50" on:click={() => dispatch('openLogs', { service: 'mysql' })}>Logs</button>
        </div>
      </div>
      <div>
        {#if $services.mysql}
          <button class="text-[12px] text-zinc-700 hover:text-black disabled:opacity-40" disabled={$loading.mysql} on:click={() => toggle('mysql')}>{$loading.mysql ? '...' : 'Stop'}</button>
        {:else}
          <button class="inline-flex items-center gap-1 rounded-full bg-black px-3.5 py-1.5 text-[12px] font-medium text-white hover:bg-zinc-800 disabled:opacity-50" disabled={$loading.mysql} on:click={() => toggle('mysql')}>{$loading.mysql ? '...' : 'Start'} <span class="text-[10px]">↗</span></button>
        {/if}
      </div>
    </div>
  {:else}
    <div class="mt-4">
      <TerminalTab />
    </div>
  {/if}

  <!-- Bottom bar always visible -->
  <div class="flex items-center justify-between border-t border-zinc-100 pt-4 mt-4">
    <button class="text-[12px] text-zinc-700 hover:text-black disabled:opacity-40" disabled={$loading.all || (!$services.apache && !$services.mysql)} on:click={handleStopAll}>Stop All</button>
    <button class="inline-flex items-center gap-1 rounded-full bg-black px-4 py-2 text-[12px] font-medium text-white hover:bg-zinc-800 disabled:opacity-50" disabled={$loading.all || allRunning} on:click={handleStartAll}>
      {#if $loading.all}<span class="w-3 h-3 border border-white/40 border-t-transparent rounded-full animate-spin"></span> Start...{:else}{allRunning ? 'All ON' : 'Start All'} <span class="text-[10px]">↗</span>{/if}
    </button>
  </div>

  {#if $lastError}
    <div class="mt-4 rounded-xl border border-red-200 bg-red-50 p-3">
      <p class="text-[11px] font-mono text-red-700 whitespace-pre-wrap break-words">{$lastError}</p>
      <div class="mt-2 flex flex-wrap gap-2">
        {#if $lastError.toLowerCase().includes('mysql') && ($lastError.toLowerCase().includes('data') || $lastError.toLowerCase().includes('corrupt') || $lastError.toLowerCase().includes('unusable') || $lastError.toLowerCase().includes('mysql_error'))}
          <button class="rounded-full bg-white border border-red-200 px-3 py-1 text-[11px] font-mono text-red-600 hover:bg-red-50" on:click={handleRepairMysql}>Repair MySQL</button>
          <button class="rounded-full bg-white border border-zinc-200 px-3 py-1 text-[11px] font-mono text-zinc-600 hover:bg-zinc-50" on:click={() => dispatch('openLogs', { service: 'mysql' })}>Buka mysql_error.log</button>
        {:else if $lastError.toLowerCase().includes('apache')}
          <button class="rounded-full bg-white border border-zinc-200 px-3 py-1 text-[11px] font-mono text-zinc-600 hover:bg-zinc-50" on:click={() => dispatch('openLogs', { service: 'apache' })}>Buka error.log</button>
        {/if}
      </div>
    </div>
  {/if}
</BezelCard>
