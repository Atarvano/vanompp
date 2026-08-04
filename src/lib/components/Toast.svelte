<script lang="ts">
  export let toasts: {id:number, msg:string}[] = []
  function remove(id:number){ toasts = toasts.filter(t=>t.id!==id) }
</script>
<div class="fixed bottom-5 right-5 z-50 flex flex-col gap-2 max-w-[360px]">
  {#each toasts as t (t.id)}
    <div class="group rounded-[0.9rem] bg-zinc-900 ring-1 ring-white/10 shadow-[0_8px_24px_rgba(0,0,0,0.5),inset_0_1px_1px_rgba(255,255,255,0.08)] px-4 py-3 flex items-start gap-2.5 animate-[slideIn_300ms_cubic-bezier(0.32,0.72,0,1)]">
      <span class="mt-0.5 w-1.5 h-1.5 bg-volt rounded-full shrink-0 shadow-[0_0_8px_rgba(233,255,112,0.5)]"></span>
      <p class="text-[13px] leading-snug text-zinc-200 flex-1">{t.msg}</p>
      <button on:click={()=>remove(t.id)} class="ml-1 w-6 h-6 rounded-full bg-white/10 flex items-center justify-center text-zinc-500 hover:text-white hover:bg-white/15">×</button>
    </div>
  {/each}
</div>
<style>
  @keyframes slideIn { from { transform: translateY(8px); opacity:0 } to { transform: translateY(0); opacity:1 } }
</style>
