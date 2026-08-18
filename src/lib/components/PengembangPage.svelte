<script lang="ts">
  import BezelCard from './BezelCard.svelte'
  import { services } from '$lib/stores/services'
  import { t } from '$lib/utils/messages'
  import { openExternal } from '$lib/utils/open'

  const REPO_URL = 'https://github.com/Atarvano/vanompp'
  const VERSION = '1.3.0'

  $: tt = $t
  $: svc = $services

  function openRepo() {
    openExternal(REPO_URL)
  }
</script>

<div class="space-y-4">
  <!-- About card -->
  <BezelCard>
    <div class="flex items-start gap-4">
      <img src="/mascot.png" alt="Vano" class="h-12 w-12 shrink-0 rounded-[12px] object-cover ring-1 ring-zinc-200 bg-white" />
      <div class="min-w-0 flex-1">
        <h2 class="text-[14px] font-semibold tracking-tight text-zinc-900">vanompp</h2>
        <p class="mt-1 text-[12px] leading-relaxed text-zinc-600">{tt('pengembangAboutDesc')}</p>
        <p class="mt-2 text-[12px] leading-relaxed text-zinc-500">{tt('pengembangForWho')}</p>
      </div>
    </div>

    <div class="mt-4 flex flex-wrap items-center gap-2 border-t border-zinc-100 pt-4">
      <span class="inline-flex items-center gap-1.5 rounded-full bg-zinc-900 px-3 py-1 text-[11px] font-medium text-white">
        {tt('pengembangCreator')}
      </span>
      <span class="text-[11px] text-zinc-400">·</span>
      <span class="text-[11px] text-zinc-500">{tt('pengembangRepo')}</span>
    </div>

    <button
      class="mt-3 inline-flex items-center gap-1 rounded-full border border-zinc-200 bg-white px-3.5 py-1.5 text-[12px] font-medium text-zinc-700 hover:bg-zinc-50 transition-colors"
      on:click={openRepo}
    >
      {tt('pengembangOpenRepo')}
    </button>
    <div class="mt-1 font-mono text-[10px] text-zinc-400">{REPO_URL}</div>
  </BezelCard>

  <!-- System info card -->
  <BezelCard>
    <h3 class="text-[11px] font-mono font-semibold tracking-[0.14em] uppercase text-zinc-500">{tt('pengembangSystemTitle')}</h3>

    <div class="mt-3 space-y-2">
      <div class="flex items-center justify-between rounded-xl bg-zinc-50 px-3 py-2.5">
        <span class="text-[11px] font-mono uppercase tracking-[0.12em] text-zinc-500">{tt('pengembangVersion')}</span>
        <span class="font-mono text-[12px] font-semibold text-zinc-900">v{VERSION}</span>
      </div>

      <div class="flex items-center justify-between rounded-xl bg-zinc-50 px-3 py-2.5">
        <span class="text-[11px] font-mono uppercase tracking-[0.12em] text-zinc-500">{tt('pengembangApache')}</span>
        <span class="flex items-center gap-2">
          {#if svc.apache}
            <span class="inline-flex items-center gap-1 rounded-full bg-[#E9FF70] px-2 py-0.5 text-[10px] font-bold text-black">
              <span class="h-1.5 w-1.5 rounded-full bg-black"></span> ON
            </span>
          {:else}
            <span class="inline-flex rounded-full bg-zinc-200 px-2 py-0.5 text-[10px] font-semibold text-zinc-500">OFF</span>
          {/if}
          <span class="font-mono text-[11px] text-zinc-600">:{svc.apachePort}</span>
        </span>
      </div>

      <div class="flex items-center justify-between rounded-xl bg-zinc-50 px-3 py-2.5">
        <span class="text-[11px] font-mono uppercase tracking-[0.12em] text-zinc-500">{tt('pengembangMysql')}</span>
        <span class="flex items-center gap-2">
          {#if svc.mysql}
            <span class="inline-flex items-center gap-1 rounded-full bg-[#E9FF70] px-2 py-0.5 text-[10px] font-bold text-black">
              <span class="h-1.5 w-1.5 rounded-full bg-black"></span> ON
            </span>
          {:else}
            <span class="inline-flex rounded-full bg-zinc-200 px-2 py-0.5 text-[10px] font-semibold text-zinc-500">OFF</span>
          {/if}
          <span class="font-mono text-[11px] text-zinc-600">:{svc.mysqlPort}</span>
        </span>
      </div>
    </div>

    <p class="mt-3 text-center font-mono text-[10px] text-zinc-400">vanompp · Apache + MySQL portable</p>
  </BezelCard>
</div>
