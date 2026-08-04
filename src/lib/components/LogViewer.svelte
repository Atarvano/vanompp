<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import BezelCard from './BezelCard.svelte'
  import { MSG } from '$lib/utils/messages'

  export let open = false
  export let service: 'apache' | 'mysql' | 'php' | null = null
  export let lines: number = 120

  const dispatch = createEventDispatcher<{ close: void; toast: { msg: string; kind?: 'info' | 'error' } }>()

  let content = ''
  let loading = false
  let err = ''
  let kind: 'error' | 'access' = 'error'
  let copied = false

  $: if (open && service) {
    loadLog()
  }

  async function loadLog() {
    if (!service) return
    loading = true
    err = ''
    content = ''
    try {
      const res = await invoke<string>('read_log', {
        service,
        lines,
        kind: kind === 'access' ? 'access' : 'error'
      })
      content = res
    } catch (e: any) {
      const msg = typeof e === 'string' ? e : e?.toString?.() || 'Gagal baca log'
      err = msg
      dispatch('toast', { msg, kind: 'error' })
    } finally {
      loading = false
    }
  }

  function close() {
    dispatch('close')
  }

  async function copyLog() {
    if (!content) return
    try {
      await navigator.clipboard.writeText(content)
      copied = true
      dispatch('toast', { msg: 'Log dicopy ke clipboard 📋', kind: 'info' })
      setTimeout(() => (copied = false), 1800)
    } catch {
      // fallback
      dispatch('toast', { msg: MSG.logReadFail('clipboard gagal'), kind: 'error' })
    }
  }

  function handleBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) close()
  }
</script>

{#if open}
  <div
    class="fixed inset-0 z-[60] bg-zinc-950/80 backdrop-blur-[6px] flex items-center justify-center p-4"
    on:click={handleBackdrop}
    on:keydown={(e)=> e.key==='Escape' && close()}
    role="presentation"
  >
    <div class="w-full max-w-[720px] max-h-[85dvh] animate-[slideIn_250ms_cubic-bezier(0.32,0.72,0,1)]">
      <BezelCard highlight>
        <div class="flex flex-col gap-4 max-h-[80dvh]">
          <div class="flex justify-between items-center">
            <div>
              <h3 class="text-[12px] font-mono font-semibold tracking-[0.14em] uppercase text-zinc-400">Logs • {service ?? 'log'}</h3>
              <p class="mt-1 text-[11px] font-mono text-zinc-600">bin/{service}/... • tail {lines} baris • <span class="text-zinc-500">volt = ON</span></p>
            </div>
            <button on:click={close} class="w-8 h-8 rounded-full bg-white/10 hover:bg-white/15 text-zinc-400 hover:text-white flex items-center justify-center">×</button>
          </div>

          <div class="flex gap-2 flex-wrap items-center">
            <div class="inline-flex rounded-full bg-zinc-800 ring-1 ring-white/10 p-1 text-[11px] font-mono">
              <button
                on:click={()=>{ service='apache'; kind='error'; loadLog() }}
                class="px-3 py-1.5 rounded-full transition-all {service==='apache' && kind==='error' ? 'bg-white text-black' : 'text-zinc-400 hover:text-white'}"
              >Apache error</button>
              <button
                on:click={()=>{ service='apache'; kind='access'; loadLog() }}
                class="px-3 py-1.5 rounded-full transition-all {service==='apache' && kind==='access' ? 'bg-white text-black' : 'text-zinc-400 hover:text-white'}"
              >access</button>
              <button
                on:click={()=>{ service='mysql'; kind='error'; loadLog() }}
                class="px-3 py-1.5 rounded-full transition-all {service==='mysql' ? 'bg-white text-black' : 'text-zinc-400 hover:text-white'}"
              >MySQL</button>
              <button
                on:click={()=>{ service='php'; kind='error'; loadLog() }}
                class="px-3 py-1.5 rounded-full transition-all {service==='php' ? 'bg-white text-black' : 'text-zinc-400 hover:text-white'}"
              >PHP</button>
            </div>
            <button on:click={loadLog} disabled={loading} class="rounded-full bg-white/[0.06] ring-1 ring-white/10 px-3 py-1.5 text-[11px] font-mono text-zinc-300 hover:bg-white/[0.10] disabled:opacity-50">
              {loading ? 'Loading...' : '↻ Refresh'}
            </button>
            <button on:click={copyLog} disabled={!content || loading} class="rounded-full bg-volt text-black px-3 py-1.5 text-[11px] font-semibold flex items-center gap-1 disabled:opacity-50">
              {copied ? 'Copied! ✓' : 'Copy'}
            </button>
          </div>

          {#if err}
            <p class="text-[11px] font-mono text-red-400 bg-red-500/10 ring-1 ring-red-500/20 rounded-xl px-3 py-2">{err}</p>
          {/if}

          <div class="rounded-[1rem] bg-zinc-950 ring-1 ring-white/10 p-3 overflow-auto max-h-[48dvh] min-h-[180px]">
            {#if loading}
              <div class="flex items-center gap-2 text-[12px] font-mono text-zinc-500 py-6 justify-center">
                <span class="w-4 h-4 border-2 border-zinc-600 border-t-volt rounded-full animate-spin"></span>
                Baca {service} log...
              </div>
            {:else if content}
              <pre class="text-[11px] font-mono leading-relaxed text-zinc-300 whitespace-pre-wrap break-words selection:bg-volt selection:text-black">{content}</pre>
            {:else}
              <p class="text-[11px] font-mono text-zinc-600 py-6 text-center">{MSG.logEmpty}</p>
            {/if}
          </div>

          <div class="flex justify-between items-center gap-2">
            <p class="text-[10px] font-mono text-zinc-600 leading-relaxed max-w-[70%]">
              Tip: kalau Apache gagal → butuh VC++ Redist. Kalau MySQL error → cek data folder. Log auto ke-refresh pas service start.
            </p>
            <button on:click={close} class="rounded-full bg-white text-black px-5 py-2 text-[12px] font-semibold hover:bg-zinc-100 active:scale-[0.98] transition-all">Tutup</button>
          </div>
        </div>
      </BezelCard>
    </div>
  </div>
{/if}

<style>
  @keyframes slideIn {
    from { transform: translateY(10px) scale(0.98); opacity:0 }
    to { transform: translateY(0) scale(1); opacity:1 }
  }
</style>
