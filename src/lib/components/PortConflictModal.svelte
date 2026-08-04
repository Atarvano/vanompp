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
    // PortInfo -> map
    const pi = c as PortInfo
    return {
      name: pi.port === 3306 ? 'mysql' as const : 'apache' as const,
      port: pi.port,
      suggest: pi.suggest
    } as ConflictInfo
  })

  $: first = list[0]
  $: title = first ? `Port ${first.port} udah dipakai` : 'Port conflict'

  function close() {
    dispatch('close')
  }

  function useSuggestFor(c: ConflictInfo) {
    dispatch('useSuggest', c)
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === 'Escape') close()
  }

  function onBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) close()
  }
</script>

<svelte:window on:keydown={onKey} />

{#if open}
  <!-- backdrop -->
  <div
    class="fixed inset-0 z-[100] flex items-center justify-center bg-black/70 backdrop-blur-[2px] p-4"
    on:click={onBackdrop}
    role="presentation"
  >
    <div
      class="w-full max-w-[480px] animate-[slideIn_220ms_cubic-bezier(0.32,0.72,0,1)]"
      role="dialog"
      aria-modal="true"
      aria-label="Port conflict"
    >
      <BezelCard highlight>
        <div class="flex justify-between items-start gap-3 mb-4">
          <div class="flex items-center gap-2.5">
            <span class="w-8 h-8 rounded-full bg-volt text-black grid place-items-center text-[14px] font-bold">!</span>
            <div>
              <h3 class="text-[13px] font-mono font-semibold tracking-[0.08em] uppercase text-white">{title} 😅</h3>
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
              <button
                on:click={() => useSuggestFor(c)}
                class="shrink-0 rounded-full bg-volt text-black px-4 py-2 text-[12px] font-semibold active:scale-[0.98] hover:bg-[#e0ff5a] transition-all"
              >
                Pakai {c.suggest}
              </button>
            </div>
          {/each}

          {#if errorMsg}
            <p class="text-[11px] font-mono text-red-400 bg-red-500/10 ring-1 ring-red-500/20 rounded-[0.75rem] px-3 py-2 leading-snug whitespace-pre-wrap break-words">{errorMsg}</p>
          {/if}

          <div class="flex gap-2 pt-1">
            <button
              on:click={() => dispatch('openLog')}
              class="rounded-full bg-white/[0.06] ring-1 ring-white/10 px-4 py-2 text-[12px] font-mono text-zinc-300 hover:bg-white/[0.10] transition-colors"
            >
              Buka error.log
            </button>
            <button
              on:click={close}
              class="rounded-full bg-white text-black px-5 py-2 text-[12px] font-semibold hover:bg-zinc-100 active:scale-[0.98] transition-all ml-auto"
            >
              Tutup
            </button>
          </div>

          <p class="text-[10px] font-mono text-zinc-600 leading-relaxed">
            Tip: tutup XAMPP/Laragon/IIS dulu, atau pakai saran port di atas. MySQL biasanya bentrok sama XAMPP di 3306.
          </p>
        </div>
      </BezelCard>
    </div>
  </div>
{/if}

<style>
  @keyframes slideIn {
    from { transform: translateY(10px) scale(0.98); opacity: 0 }
    to { transform: translateY(0) scale(1); opacity: 1 }
  }
</style>
