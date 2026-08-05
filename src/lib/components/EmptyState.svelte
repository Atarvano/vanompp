<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import BezelCard from './BezelCard.svelte'
  const dispatch = createEventDispatcher()
</script>

<BezelCard>
  <div class="py-6 md:py-8 text-center">
    <div class="mx-auto flex max-w-md flex-col items-center gap-4">
      <img src="/mascot.png" alt="Vano mascot" class="h-24 w-24 rounded-[20px] ring-1 ring-zinc-200 shadow-sm" />
      <div class="space-y-1">
        <h3 class="text-sm font-semibold text-zinc-900">Belum ada project</h3>
        <p class="text-[12px] leading-relaxed text-zinc-500">
          Folder <code class="rounded bg-zinc-100 px-1.5 py-0.5 font-mono text-zinc-700">www</code> masih kosong.
          Klik di bawah buat project pertama — nanti link-nya muncul di atas biar gampang buka.
        </p>
      </div>
      <button
        on:click={() => dispatch('cta')}
        class="mt-1 rounded-full bg-black px-5 py-2.5 text-[12px] font-medium text-white hover:bg-zinc-800 active:scale-[0.98] transition-all"
      >
        + Buat Project Pertamaku
      </button>
    </div>
  </div>
</BezelCard>
