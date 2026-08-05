<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import BezelCard from './BezelCard.svelte'
  import type { ConflictInfo, PortInfo } from '$lib/stores/services'

  export let open = false
  export let conflicts: (ConflictInfo | PortInfo)[] = []
  export let errorMsg: string = ''

  const dispatch = createEventDispatcher<{
    close: void
    useSuggest: { name: 'apache' | 'mysql'; port: number; suggest: number }
    openLog: void
  }>()

  $: list = conflicts.map((c: any) => {
    if ('name' in c) return c as ConflictInfo
    const pi = c as PortInfo
    return {
      name: pi.port === 3306 || pi.port === 3307 || (pi.port >= 3309 && pi.port <= 3320) ? 'mysql' as const : 'apache' as const,
      port: pi.port,
      suggest: pi.suggest
    } as ConflictInfo
  })

  $: first = list[0]
  $: title = first ? `Port ${first.port} udah dipakai 😅` : 'Port conflict'

  function close() {
    dispatch('close')
  }
  function useSuggestFor(c: ConflictInfo) {
    dispatch('useSuggest', c)
  }
</script>

{#if open}
  <div
    class="fixed inset-0 z-[100] bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
    on:click|self={close}
    on:keydown={(e) => { if (e.key === 'Escape') close() }}
    role="dialog"
    aria-modal="true"
    aria-label="Port conflict"
    tabindex="-1"
  >
    <BezelCard highlight>
      <div class="flex justify-between items-start gap-3 mb-4">
        <div class="flex items-center gap-2.5">
          <span class="w-8 h-8 rounded-full bg-volt text-black grid place-items-center text-[14px] font-bold">!</span>
          <div>
            <h3 class="text-[13px] font-mono font-semibold tracking-[0.08em] uppercase text-white">{title}</h3>
            <p class="text-[11px] font-mono text-zinc-500 mt-0.5">coba port lain yang masih kosong</p>
          </div>
        </div>
        <button
          on:click={close}
          class="w-7 h-7 rounded-full bg-white/[0.06] ring-1 ring-white/10 grid place-items-center text-zinc-500 hover:text-white hover:bg-white/10 transition-colors"
          aria-label="Tutup"
        >×</button>
      </div>

      <div class="space-y-3">
        {#each list as c}
          <div class="rounded-[1rem] bg-white/[0.04] ring-1 ring-white/10 px-4 py-3 flex items-center justify-between gap-3">
            <div class="min-w-0">
              <p class="text-[12px] font-mono text-zinc-300">
                <span class="text-white font-semibold">{c.name.toUpperCase()}</span>
                <span class="text-zinc-600 mx-1">•</span>
                port <code class="bg-white/[0.07] px-1.5 py-0.5 rounded-full text-[11px] text-zinc-300">{c.port}</code>
                <span class="text-zinc-600 mx-1">kepake</span>
                → saran <code class="bg-volt/15 ring-1 ring-volt/20 px-1.5 py-0.5 rounded-full text-[11px] text-volt">{c.suggest}</code>
              </p>
            </div>
          </div>
        {/each}

        {#if errorMsg}
          <p class="text-[11px] font-mono text-red-400 bg-red-500/10 ring-1 ring-red-500/20 rounded-[0.75rem] px-3 py-2 leading-snug whitespace-pre-wrap break-words">{errorMsg}</p>
        {/if}

        {#if first}
          <!-- Q4=Y single button Tetap pakai {suggest} = Pakai & Ingat forever VOLT fullwidth -->
          <button
            on:click={() => useSuggestFor(first)}
            class="w-full rounded-full bg-[#E9FF70] text-black px-5 py-3 text-[13px] font-semibold tracking-tight hover:brightness-[1.03] active:scale-[0.98] transition-all flex items-center justify-center gap-2"
          >
            Tetap pakai {first.suggest}
            <span class="w-6 h-6 bg-black/10 rounded-full grid place-items-center text-[11px]">→</span>
          </button>
          <p class="text-[10px] font-mono text-zinc-500 text-center -mt-1">Ingat terus ya — kamu gak bakal ditanya lagi soal ini</p>
        {/if}

        <div class="flex gap-2">
          <button
            on:click={() => dispatch('openLog')}
            class="flex-1 rounded-full bg-white/[0.06] ring-1 ring-white/10 px-4 py-2.5 text-[12px] font-mono text-zinc-300 hover:text-white hover:bg-white/[0.10] transition-colors"
          >
            Buka error.log
          </button>
          <button
            on:click={close}
            class="flex-1 rounded-full bg-white/[0.03] ring-1 ring-white/10 px-4 py-2.5 text-[12px] font-mono text-zinc-500 hover:text-zinc-300 transition-colors"
          >
            Tutup
          </button>
        </div>

        <p class="text-[10px] font-mono text-zinc-600 leading-relaxed">
          Tip: tutup XAMPP/Laragon/IIS dulu, atau klik Tetap pakai di atas. MySQL biasanya bentrok sama XAMPP di 3306.
        </p>
      </div>
    </BezelCard>
  </div>
{/if}

<style>
  @keyframes slideIn {
    from { transform: translateY(10px) scale(0.98); opacity: 0 }
    to { transform: translateY(0) scale(1); opacity: 1 }
  }
</style>
