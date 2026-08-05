<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import type { ConflictInfo, PortInfo } from '$lib/stores/services'

  export let open = false
  export let conflicts: (ConflictInfo | PortInfo)[] = []
  export let errorMsg = ''

  const dispatch = createEventDispatcher<{
    close: void
    useSuggest: { name: 'apache' | 'mysql'; port: number; suggest: number }
    openLog: void
  }>()

  $: list = (conflicts as any[]).map((c: any) => {
    if (c.name && c.port != null) return c as ConflictInfo
    const pi = c as PortInfo
    const isMysql = pi.port === 3306 || pi.port === 3307 || (pi.port >= 3309 && pi.port <= 3320)
    return { name: isMysql ? 'mysql' as const : 'apache' as const, port: pi.port, suggest: pi.suggest } as ConflictInfo
  })
</script>

{#if open}
  <div
    class="fixed inset-0 z-[60] flex items-center justify-center bg-black/20 p-4 backdrop-blur-[2px]"
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    on:click|self={() => dispatch('close')}
    on:keydown={(e) => { if (e.key === 'Escape') dispatch('close') }}
  >
    <div class="w-full max-w-[460px] rounded-[24px] border border-zinc-200 bg-white p-6 shadow-xl">
      <div class="flex items-center justify-between">
        <h3 class="text-sm font-semibold text-zinc-900">Port kepake, geser?</h3>
        <button class="text-zinc-400 hover:text-zinc-700" on:click={() => dispatch('close')}>x</button>
      </div>

      {#if errorMsg}
        <p class="mt-3 rounded-xl border border-red-200 bg-red-50 px-3 py-2 font-mono text-[11px] text-red-700 whitespace-pre-wrap break-words">{errorMsg}</p>
      {/if}

      <div class="mt-4 space-y-3">
        {#each list as c}
          <div class="flex items-center justify-between rounded-xl border border-zinc-100 bg-zinc-50/60 px-3 py-2.5">
            <div class="flex items-center gap-2">
              <span class="font-mono text-[10px] uppercase tracking-widest text-zinc-500">{c.name}</span>
              <span class="font-mono text-[11px] text-zinc-700">{c.port} -> {c.suggest}</span>
            </div>
            <button
              class="rounded-full bg-black px-3.5 py-1.5 text-[11px] font-medium text-white hover:bg-zinc-800 transition-colors"
              on:click={() => dispatch('useSuggest', c)}
            >
              Tetap pakai {c.suggest}
            </button>
          </div>
        {/each}
      </div>

      <div class="mt-5 flex justify-between gap-2">
        <button
          class="rounded-full border border-zinc-200 px-4 py-2 text-[12px] text-zinc-700 hover:bg-zinc-50"
          on:click={() => dispatch('openLog')}
        >
          Buka Logs
        </button>
        <button
          class="rounded-full bg-white border border-zinc-200 px-4 py-2 text-[12px] text-zinc-700 hover:bg-zinc-50"
          on:click={() => dispatch('close')}
        >
          Tutup
        </button>
      </div>

      <p class="mt-4 text-[10px] font-mono leading-relaxed text-zinc-500">
        Tip: tutup XAMPP/Laragon/IIS dulu, atau klik Tetap pakai di atas. MySQL biasanya bentrok sama XAMPP di 3306.
      </p>
    </div>
  </div>
{/if}
