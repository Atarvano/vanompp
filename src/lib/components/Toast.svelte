<script lang="ts">
  import { onDestroy } from 'svelte'

  export let toasts: { id: number; msg: string; kind?: 'info' | 'error' }[] = []
  export let autoMs = 4000

  const timers = new Map<number, ReturnType<typeof setTimeout>>()

  $: {
    for (const t of toasts) {
      if (!timers.has(t.id)) {
        const id = setTimeout(() => {
          toasts = toasts.filter((x) => x.id !== t.id)
          timers.delete(t.id)
        }, autoMs)
        timers.set(t.id, id)
      }
    }
    for (const id of [...timers.keys()]) {
      if (!toasts.find((x) => x.id === id)) {
        const tm = timers.get(id)
        if (tm) clearTimeout(tm)
        timers.delete(id)
      }
    }
  }

  function remove(id: number) {
    toasts = toasts.filter((t) => t.id !== id)
    const tm = timers.get(id)
    if (tm) clearTimeout(tm)
    timers.delete(id)
  }

  onDestroy(() => {
    for (const tm of timers.values()) clearTimeout(tm)
    timers.clear()
  })
</script>

<div class="pointer-events-none fixed bottom-4 right-4 z-50 flex max-w-[360px] flex-col gap-2">
  {#each toasts as t (t.id)}
    <div
      class="pointer-events-auto flex items-start justify-between gap-3 rounded-[16px] border bg-white px-4 py-3 text-[12px] shadow-[0_8px_24px_rgba(0,0,0,0.08)] animate-[in_0.2s_ease] {t.kind === 'error'
        ? 'border-red-200 bg-red-50 text-red-800'
        : 'border-zinc-200 text-zinc-800'}"
    >
      <p class="flex-1 leading-snug">{t.msg}</p>
      <button class="ml-1 text-zinc-400 hover:text-zinc-700" on:click={() => remove(t.id)}>x</button>
    </div>
  {/each}
</div>

<style>
  @keyframes in {
    from { opacity: 0; transform: translateY(6px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
