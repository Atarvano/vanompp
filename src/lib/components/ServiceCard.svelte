<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { get } from 'svelte/store'
  import BezelCard from './BezelCard.svelte'
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
    type ConflictInfo
  } from '$lib/stores/services'
  import { MSG } from '$lib/utils/messages'

  const dispatch = createEventDispatcher<{
    conflict: { conflicts: ConflictInfo[]; error?: string }
    error: { msg: string }
    toast: { msg: string }
    openLogs: { service: 'apache' | 'mysql' | 'php' }
  }>()

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
        const m = typeof e === 'string' ? e : String(e)
        dispatch('error', { msg: m })
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
    } catch {
      // ignore
    }

    try {
      const portToUse = name === 'apache' ? svc.apachePort : svc.mysqlPort
      const msg = await startService(name, portToUse)
      dispatch('toast', { msg: `${name.toUpperCase()} nyala — ${msg}` })
    } catch (e) {
      const msg = typeof e === 'string' ? e : String(e)
      const lower = msg.toLowerCase()
      if (
        lower.includes('port') ||
        lower.includes('dipakai') ||
        lower.includes('kepake') ||
        lower.includes('already') ||
        lower.includes('in use') ||
        lower.includes('conflict')
      ) {
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
    if (ld.all || ld.apache || ld.mysql) return
    try {
      const infos = await checkPorts()
      const conflicts = toConflicts(infos).filter((c) => {
        if (c.name === 'apache' && svc.apache) return false
        if (c.name === 'mysql' && svc.mysql) return false
        return true
      })
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
      const lower = msg.toLowerCase()
      if (lower.includes('port') || lower.includes('dipakai') || lower.includes('kepake') || lower.includes('in use')) {
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
      const msg = typeof e === 'string' ? e : String(e)
      dispatch('error', { msg })
    }
  }
</script>

<BezelCard highlight={anyRunning}>
  <div class="flex justify-between items-start gap-4">
    <div class="min-w-0 flex-1">
      <h2 class="text-[11px] font-mono font-semibold tracking-[0.14em] uppercase text-zinc-500 mb-3 flex items-center gap-2">
        Services
        {#if ld.all}
          <span class="inline-flex items-center gap-1 text-[10px] text-zinc-400">
            <span class="w-3 h-3 border border-zinc-500 border-t-transparent rounded-full animate-spin"></span>
            proses...
          </span>
        {/if}
      </h2>

      <div class="flex gap-2.5 flex-wrap">
        <button
          on:click={() => toggle('apache')}
          disabled={ld.apache || ld.all}
          class="group flex items-center gap-2 rounded-full ring-1 px-3 py-1.5 text-[12px] font-mono transition-all active:scale-[0.98] disabled:opacity-60 disabled:cursor-not-allowed
            {svc.apache ? 'bg-volt/15 ring-volt/25 hover:ring-volt/40' : 'bg-white/[0.04] ring-white/10 hover:ring-white/20'}"
        >
          {#if ld.apache}
            <span class="w-2 h-2 rounded-full bg-zinc-500 animate-pulse"></span>
            <span class="w-3 h-3 border border-zinc-500 border-t-transparent rounded-full animate-spin"></span>
          {:else}
            <span class="w-2 h-2 rounded-full {svc.apache ? 'bg-volt shadow-[0_0_8px_rgba(233,255,112,0.6)]' : 'bg-zinc-600'} transition-colors"></span>
          {/if}
          <span class="{svc.apache ? 'text-volt' : 'text-zinc-300'}">Apache</span>
          <span class="ml-0.5 text-[10px] {svc.apache ? 'text-volt' : 'text-zinc-600'} font-bold">{svc.apache ? 'ON' : 'OFF'}</span>
        </button>

        <button
          on:click={() => toggle('mysql')}
          disabled={ld.mysql || ld.all}
          class="group flex items-center gap-2 rounded-full ring-1 px-3 py-1.5 text-[12px] font-mono transition-all active:scale-[0.98] disabled:opacity-60 disabled:cursor-not-allowed
            {svc.mysql ? 'bg-volt/15 ring-volt/25 hover:ring-volt/40' : 'bg-white/[0.04] ring-white/10 hover:ring-white/20'}"
        >
          {#if ld.mysql}
            <span class="w-2 h-2 rounded-full bg-zinc-500 animate-pulse"></span>
            <span class="w-3 h-3 border border-zinc-500 border-t-transparent rounded-full animate-spin"></span>
          {:else}
            <span class="w-2 h-2 rounded-full {svc.mysql ? 'bg-volt shadow-[0_0_8px_rgba(233,255,112,0.6)]' : 'bg-zinc-600'} transition-colors"></span>
          {/if}
          <span class="{svc.mysql ? 'text-volt' : 'text-zinc-300'}">MySQL</span>
          <span class="ml-0.5 text-[10px] {svc.mysql ? 'text-volt' : 'text-zinc-600'} font-bold">{svc.mysql ? 'ON' : 'OFF'}</span>
        </button>

        <button
          on:click={()=>dispatch('openLogs',{service:'apache'})}
          class="rounded-full bg-white/[0.04] ring-1 ring-white/10 px-3 py-1.5 text-[11px] font-mono text-zinc-500 hover:text-zinc-300 hover:ring-white/20 active:scale-[0.98] transition-all"
        >Logs</button>
      </div>

      <div class="mt-2.5 flex flex-wrap gap-x-3 gap-y-1 text-[10px] font-mono text-zinc-600">
        <span class="inline-flex items-center gap-1">
          # apache:{svc.apachePort}
          {#if svc.apachePid}<span class="text-zinc-500">pid {svc.apachePid}</span>{/if}
          {#if !svc.apachePortFree && !svc.apache}<span class="text-red-400">kepake</span>{/if}
        </span>
        <span class="inline-flex items-center gap-1">
          # mysql:{svc.mysqlPort}
          {#if svc.mysqlPid}<span class="text-zinc-500">pid {svc.mysqlPid}</span>{/if}
          {#if !svc.mysqlPortFree && !svc.mysql}<span class="text-red-400">kepake</span>{/if}
        </span>
      </div>

      {#if err}
        <p class="mt-2 text-[11px] font-mono text-red-400 bg-red-500/10 ring-1 ring-red-500/20 rounded-[0.6rem] px-2.5 py-1.5 leading-snug break-words">{err}</p>
      {/if}

      {#if err && err.toLowerCase().includes('vc++')}
        <div class="mt-2 flex gap-2">
          <button on:click={()=>dispatch('openLogs',{service:'apache'})} class="rounded-full bg-red-500/15 ring-1 ring-red-500/20 px-3 py-1 text-[11px] font-mono text-red-300 hover:bg-red-500/20">Buka Logs</button>
          <span class="text-[10px] font-mono text-zinc-500 py-1">{MSG.vcRedistTip}</span>
        </div>
      {/if}
    </div>

    <div class="flex flex-col gap-2 items-end shrink-0">
      {#if anyRunning}
        <button
          on:click={handleStopAll}
          disabled={ld.all}
          class="rounded-full bg-white/[0.06] ring-1 ring-white/10 px-4 py-2.5 text-[12px] font-mono text-zinc-300 hover:bg-white/[0.10] active:scale-[0.98] transition-all flex items-center gap-2 disabled:opacity-50"
        >
          {#if ld.all}
            <span class="w-3.5 h-3.5 border-2 border-zinc-500 border-t-transparent rounded-full animate-spin"></span>
            Stop...
          {:else}
            Stop All
          {/if}
        </button>
      {/if}

      <button
        on:click={handleStartAll}
        disabled={ld.all || allRunning}
        class="rounded-full px-5 py-2.5 text-sm font-semibold flex items-center gap-2 active:scale-[0.98] transition-all duration-[600ms] ease-[cubic-bezier(0.32,0.72,0,1)]
          {allRunning ? 'bg-zinc-800 text-zinc-600 cursor-not-allowed' : 'bg-white text-black hover:bg-zinc-100'} disabled:opacity-60"
      >
        {#if ld.all}
          <span class="w-4 h-4 border-2 border-zinc-500 border-t-transparent rounded-full animate-spin"></span>
          Start...
        {:else}
          {allRunning ? 'All ON' : 'Start All'}
          <span class="w-7 h-7 bg-black/10 rounded-full flex items-center justify-center text-[12px]">{allRunning ? '✓' : '↗'}</span>
        {/if}
      </button>
    </div>
  </div>
</BezelCard>
