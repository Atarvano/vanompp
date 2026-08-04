<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import BezelCard from './BezelCard.svelte'
  const dispatch = createEventDispatcher()
</script>
<BezelCard>
  <div class="text-center py-6 md:py-10">
    <div class="mx-auto max-w-md">
      <div class="flex items-center justify-center gap-3 mb-6 font-mono text-[12px]">
        <div class="rounded-[0.75rem] border border-dashed border-white/15 bg-white/[0.03] px-3 py-2 text-zinc-400">www/</div>
        <span class="text-zinc-600">→</span>
        <div class="rounded-[0.75rem] ring-1 ring-volt/20 bg-volt/10 px-3 py-2 text-volt">localhost:8080/{'{folder}'}</div>
      </div>
      <p class="text-sm text-zinc-400 leading-relaxed">Belum ada project. Folder <code class="bg-white/[0.06] px-1.5 py-0.5 rounded text-zinc-300">www</code> masih kosong. Buat project pertamamu, nanti URL-nya bakal muncul gede di sini biar ga bingung.</p>
      <button on:click={()=>dispatch('cta')} class="mt-6 rounded-full bg-volt text-black px-6 py-3 text-sm font-semibold flex items-center gap-2 mx-auto active:scale-[0.98] transition-transform duration-[600ms] ease-[cubic-bezier(0.32,0.72,0,1)] hover:bg-[#e0ff5a]">
        + Buat Project Pertamaku
        <span class="w-7 h-7 bg-black/10 rounded-full flex items-center justify-center text-xs">↗</span>
      </button>
    </div>
  </div>
</BezelCard>
