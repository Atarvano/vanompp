<script lang="ts">
  export let toasts: {id:number, msg:string, kind?: 'info'|'error'}[] = []
  export let autoMs = 4000
  let timers = new Map<number, ReturnType<typeof setTimeout>>()

  $: {
    // schedule auto-dismiss for new toasts
    for (const t of toasts) {
      if (!timers.has(t.id)) {
        const id = setTimeout(() => remove(t.id), autoMs)
        timers.set(t.id, id)
      }
    }
    // cleanup removed
    for (const id of [...timers.keys()]) {
      if (!toasts.find(x=>x.id===id)) {
        clearTimeout(timers.get(id)!)
        timers.delete(id)
      }
    }
  }

  function remove(id:number){
    const tm = timers.get(id)
    if (tm) { clearTimeout(tm); timers.delete(id) }
    toasts = toasts.filter(t=>t.id!==id)
  }
</script>

<div class="fixed bottom-5 right-5 z-[100] flex flex-col gap-2.5 max-w-[380px] pointer-events-none">
  {#each toasts as t (t.id)}
    <div
      class="pointer-events-auto group rounded-[0.9rem] bg-zinc-900 ring-1 ring-white/10 shadow-[0_10px_28px_rgba(0,0,0,0.55),inset_0_1px_1px_rgba(255,255,255,0.08)] px-4 py-3 flex items-start gap-2.5 animate-[slideIn_300ms_cubic-bezier(0.32,0.72,0,1)]
             border-l-[3px] {t.kind==='error' ? 'border-l-red-400 !ring-red-500/10' : 'border-l-volt'}"
    >
      <span class="mt-0.5 w-2 h-2 rounded-full shrink-0
        {t.kind==='error' ? 'bg-red-400 shadow-[0_0_8px_rgba(248,113,113,0.6)]' : 'bg-volt shadow-[0_0_8px_rgba(233,255,112,0.5)]'}"
      ></span>
      <p class="text-[13px] leading-snug flex-1 {t.kind==='error' ? 'text-red-200' : 'text-zinc-200'}">{t.msg}</p>
      <button on:click={()=>remove(t.id)} class="ml-1 w-6 h-6 rounded-full bg-white/10 flex items-center justify-center text-zinc-500 hover:text-white hover:bg-white/15 transition-colors">×</button>
    </div>
  {/each}
</div>

<style>
  @keyframes slideIn { from { transform: translateY(8px) scale(0.98); opacity:0 } to { transform: translateY(0) scale(1); opacity:1 } }
</style>
