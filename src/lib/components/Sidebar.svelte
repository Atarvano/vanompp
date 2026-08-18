<script lang="ts">
  import { activePage, sidebarCollapsed, type Page } from '$lib/stores/nav'
  import { locale, setLocale } from '$lib/stores/locale'
  import { t } from '$lib/utils/messages'

  const items: { id: Page; icon: string }[] = [
    { id: 'services', icon: '◉' },
    { id: 'projects', icon: '▭' },
    { id: 'pengembang', icon: '◐' },
  ]

  $: collapsed = $sidebarCollapsed
  $: current = $activePage
  $: lang = $locale
  $: tt = $t

  function nav(id: Page) {
    activePage.set(id)
  }
  function toggleCollapse() {
    sidebarCollapsed.update((v) => !v)
  }
  function toggleLang() {
    setLocale(lang === 'id' ? 'en' : 'id')
  }
</script>

<!-- desktop sidebar -->
<aside
  class="hidden md:flex flex-col shrink-0 border-r border-zinc-200 bg-white transition-[width] duration-200 ease-[cubic-bezier(0.32,0.72,0,1)] {collapsed
    ? 'w-[48px]'
    : 'w-[200px]'}"
  aria-label="Sidebar"
>
  <!-- brand -->
  <div class="flex h-[52px] items-center gap-2.5 border-b border-zinc-100 px-3">
    <img src="/mascot.png" alt="Vano" class="h-7 w-7 shrink-0 rounded-[8px] object-cover ring-1 ring-zinc-200 bg-white" />
    {#if !collapsed}
      <div class="min-w-0 leading-tight">
        <div class="text-[13px] font-bold tracking-tight text-zinc-900">vanompp</div>
        <div class="truncate text-[10px] text-zinc-500">{tt('appSubtitle')}</div>
      </div>
    {/if}
  </div>

  <!-- nav -->
  <nav class="flex-1 space-y-1 p-2" aria-label="Main">
    {#each items as it}
      {@const active = current === it.id}
      <button
        class="flex w-full items-center gap-2.5 rounded-[12px] px-2.5 py-2 text-left text-[13px] transition-colors
          {active ? 'bg-zinc-900 text-white' : 'text-zinc-600 hover:bg-zinc-100 hover:text-zinc-900'}
          {collapsed ? 'justify-center' : ''}"
        on:click={() => nav(it.id)}
        aria-current={active ? 'page' : undefined}
        title={tt('nav' + it.id.charAt(0).toUpperCase() + it.id.slice(1))}
      >
        <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-[8px] text-[11px] font-mono {active ? 'bg-white/15' : 'bg-zinc-100'}"
          >{it.icon}</span
        >
        {#if !collapsed}
          <span class="truncate font-medium">{tt('nav' + it.id.charAt(0).toUpperCase() + it.id.slice(1))}</span>
        {/if}
      </button>
    {/each}
  </nav>

  <!-- bottom: lang + collapse + version -->
  <div class="border-t border-zinc-100 p-2 space-y-2">
    <button
      class="flex w-full items-center gap-2 rounded-[10px] border border-zinc-200 bg-white px-2.5 py-2 text-[11px] font-mono text-zinc-700 hover:bg-zinc-50 {collapsed
        ? 'justify-center px-1'
        : ''}"
      on:click={toggleLang}
      title={tt('langLabel')}
      aria-label={tt('langLabel')}
    >
      {#if collapsed}
        <span class="text-[11px] font-bold">{lang.toUpperCase()}</span>
      {:else}
        <span class="text-zinc-500">{tt('langLabel')}</span>
        <span class="ml-auto rounded-full bg-zinc-900 px-2 py-0.5 text-[10px] font-bold text-white">{lang.toUpperCase()}</span>
      {/if}
    </button>

    <div class="flex items-center {collapsed ? 'justify-center' : 'justify-between'} gap-1">
      {#if !collapsed}
        <span class="font-mono text-[10px] text-zinc-400">v1.3.0</span>
      {/if}
      <button
        class="flex h-7 w-7 items-center justify-center rounded-full border border-zinc-200 bg-white text-zinc-600 hover:bg-zinc-50"
        on:click={toggleCollapse}
        aria-label={collapsed ? tt('expand') : tt('collapse')}
        title={collapsed ? tt('expand') : tt('collapse')}
      >
        <span class="text-[12px]">{collapsed ? '»' : '«'}</span>
      </button>
    </div>
    {#if collapsed}
      <div class="text-center font-mono text-[9px] text-zinc-400">v1.3.0</div>
    {/if}
  </div>
</aside>

<!-- mobile bottom nav -->
<nav class="flex md:hidden fixed bottom-0 inset-x-0 z-30 border-t border-zinc-200 bg-white/95 backdrop-blur supports-[backdrop-filter]:bg-white/80">
  {#each items as it}
    {@const active = current === it.id}
    <button
      class="flex flex-1 flex-col items-center gap-0.5 py-2.5 text-[11px] {active ? 'text-zinc-900 font-semibold' : 'text-zinc-500'}"
      on:click={() => nav(it.id)}
      aria-current={active ? 'page' : undefined}
    >
      <span class="text-[14px] leading-none">{it.icon}</span>
      <span class="font-mono text-[10px] tracking-wide">{tt('nav' + it.id.charAt(0).toUpperCase() + it.id.slice(1))}</span>
    </button>
  {/each}
  <button class="flex flex-col items-center gap-0.5 py-2.5 px-3 text-zinc-500" on:click={toggleLang} aria-label={tt('langLabel')}>
    <span class="rounded-full bg-zinc-900 px-2 py-0.5 text-[10px] font-bold text-white">{lang.toUpperCase()}</span>
    <span class="font-mono text-[9px]">{tt('langLabel')}</span>
  </button>
</nav>
