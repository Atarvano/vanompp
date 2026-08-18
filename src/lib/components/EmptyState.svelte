<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import BezelCard from './BezelCard.svelte'
  import { t } from '$lib/utils/messages'
  $: tt = $t
  const dispatch = createEventDispatcher()
</script>

<BezelCard>
  <div class="py-6 md:py-8 text-center">
    <div class="mx-auto flex max-w-md flex-col items-center gap-4">
      <img src="/mascot.png" alt="Vano mascot" class="h-24 w-24 rounded-[20px] ring-1 ring-zinc-200 shadow-sm" />
      <div class="space-y-1">
        <h3 class="text-sm font-semibold text-zinc-900">{tt('emptyTitle')}</h3>
        <p class="text-[12px] leading-relaxed text-zinc-500">
          {tt('emptyDesc')}
        </p>
      </div>
      <button
        on:click={() => dispatch('cta')}
        class="mt-1 rounded-full bg-black px-5 py-2.5 text-[12px] font-medium text-white hover:bg-zinc-800 active:scale-[0.98] transition-all"
      >
        {tt('emptyCta')}
      </button>
    </div>
  </div>
</BezelCard>
