<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import { MSG } from '$lib/utils/messages'
  export let open = false
  export let service: 'apache' | 'mysql' | 'php' | null = null
  const dispatch = createEventDispatcher<{ close: void; toast: { msg: string; kind?: 'info' | 'error' } }>()

  let content = ''
  let loading = false
  let err = ''
  let kind: 'error' | 'access' = 'error'
  let activeService: 'apache' | 'mysql' | 'php' = 'apache'

  $: if (service) activeService = service as any
  $: if (open && activeService) loadLog()

  async function loadLog() {
    loading = true
    err = ''
    content = ''
    try {
      const res = await invoke<string>('read_log', {
        service: activeService,
        kind: kind === 'access' ? 'access' : 'error',
        lines: 120
      } as any)
      content = res || '(log kosong)'
    } catch (e) {
      const raw = typeof e === 'string' ? e : (e as any)?.message ?? String(e)
      err = MSG.logReadFail(raw)
      dispatch('toast', { msg: err, kind: 'error' })
    } finally {
      loading = false
    }
  }

  async function copyAll() {
    try {
      await navigator.clipboard.writeText(content)
      dispatch('toast', { msg: 'Log dicopy', kind: 'info' })
    } catch {
      dispatch('toast', { msg: MSG.logReadFail('clipboard gagal'), kind: 'error' })
    }
  }

  async function openLogsFolder() {
    try {
      await invoke('open_logs_folder')
    } catch (e) {
      dispatch('toast', { msg: `Gagal buka folder logs: ${String(e)}`, kind: 'error' })
    }
  }
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/30 backdrop-blur-sm p-4" on:click|self={() => dispatch('close')}>
    <div class="bg-white border border-zinc-200 rounded-[24px] p-5 md:p-6 shadow-[0_16px_48px_rgba(0,0,0,0.15)] w-full max-w-[640px] max-h-[85vh] flex flex-col">
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-3">
          <h3 class="text-sm font-semibold text-zinc-900">{activeService === 'apache' ? 'Apache error.log' : activeService === 'mysql' ? 'MySQL mysql_error.log' : 'PHP error'}</h3>
          <div class="flex items-center gap-1.5">
            <button
              on:click={()=>{ activeService='apache'; kind='error'; loadLog() }}
              class="px-3 py-1.5 rounded-full text-[11px] font-medium transition-all {activeService==='apache' && kind==='error' ? 'bg-zinc-900 text-white' : 'text-zinc-500 hover:text-zinc-700 bg-zinc-100'}"
            >Apache error</button>
            <button
              on:click={()=>{ activeService='mysql'; kind='error'; loadLog() }}
              class="px-3 py-1.5 rounded-full text-[11px] font-medium transition-all {activeService==='mysql' && kind==='error' ? 'bg-zinc-900 text-white' : 'text-zinc-500 hover:text-zinc-700 bg-zinc-100'}"
            >MySQL error</button>
            <button
              on:click={()=>{ activeService='php'; kind='error'; loadLog() }}
              class="px-3 py-1.5 rounded-full text-[11px] font-medium transition-all {activeService==='php' ? 'bg-zinc-900 text-white' : 'text-zinc-500 hover:text-zinc-700 bg-zinc-100'}"
            >PHP</button>
          </div>
        </div>
        <button class="h-7 w-7 rounded-full border border-zinc-200 bg-white text-zinc-500 hover:bg-zinc-50" on:click={()=>dispatch('close')}>x</button>
      </div>

      <div class="flex items-center justify-between gap-2 mb-3">
        <div class="text-[10px] font-mono text-zinc-500">120 baris terakhir</div>
        <div class="flex items-center gap-2">
          <button class="rounded-full border border-zinc-200 bg-white px-3 py-1 text-[11px] text-zinc-700 hover:bg-zinc-50" on:click={copyAll}>Copy</button>
          <button class="rounded-full border border-zinc-200 bg-white px-3 py-1 text-[11px] text-zinc-700 hover:bg-zinc-50" on:click={openLogsFolder}>Buka folder logs</button>
          <button class="rounded-full bg-zinc-100 px-3 py-1 text-[11px] text-zinc-700 hover:bg-zinc-200" on:click={loadLog}>Refresh</button>
        </div>
      </div>

      {#if loading}
        <div class="rounded-xl bg-zinc-50 border border-zinc-200 p-4 font-mono text-[11px] text-zinc-500">Loading log...</div>
      {:else if err}
        <div class="rounded-xl bg-red-50 border border-red-200 p-4 font-mono text-[11px] text-red-700">{err}</div>
      {:else}
        <pre class="flex-1 overflow-auto rounded-xl bg-zinc-50 border border-zinc-200 p-3 font-mono text-[11px] text-zinc-700 leading-relaxed whitespace-pre-wrap break-words max-h-[50vh]">{content}</pre>
      {/if}

      <p class="mt-3 text-[10px] font-mono text-zinc-400">Tip: Apache gagal butuh VC++ Redist. MySQL error cek data folder.</p>

      <div class="mt-4 flex justify-end">
        <button class="rounded-full bg-black px-4 py-2 text-[12px] font-medium text-white hover:bg-zinc-800" on:click={()=>dispatch('close')}>Tutup</button>
      </div>
    </div>
  </div>
{/if}
