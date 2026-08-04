<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { get } from 'svelte/store'
  import BrandWordmark from './lib/components/BrandWordmark.svelte'
  import ServiceCard from './lib/components/ServiceCard.svelte'
  import ProjectCard from './lib/components/ProjectCard.svelte'
  import CreateCard from './lib/components/CreateCard.svelte'
  import EmptyState from './lib/components/EmptyState.svelte'
  import Toast from './lib/components/Toast.svelte'
  import PortConflictModal from './lib/components/PortConflictModal.svelte'
  import LogViewer from './lib/components/LogViewer.svelte'
  import { projects, refreshProjects } from './lib/stores/projects'
  import { services, refreshStatus, startService, type ConflictInfo } from './lib/stores/services'
  import { MSG } from './lib/utils/messages'

  type ToastItem = { id: number; msg: string; kind?: 'info' | 'error' }
  let toasts: ToastItem[] = []
  let nextToastId = 1

  // Port conflict modal state
  let modalOpen = false
  let modalConflicts: ConflictInfo[] = []
  let modalError = ''

  // LogViewer state
  let logOpen = false
  let logService: 'apache' | 'mysql' | 'php' | null = null

  // polling
  let pollTimer: ReturnType<typeof setInterval> | null = null
  const POLL_MS = 3000

  $: projectList = $projects
  $: isEmpty = projectList.length === 0
  $: apachePort = $services.apachePort

  onMount(async () => {
    await Promise.all([refreshStatus(), refreshProjects()])
    pollTimer = setInterval(() => { refreshStatus() }, POLL_MS)
  })

  onDestroy(() => {
    if (pollTimer) clearInterval(pollTimer)
  })

  function addToast(msg: string, kind: 'info' | 'error' = 'info') {
    const id = nextToastId++
    toasts = [...toasts, { id, msg, kind } as any]
    // Toast component auto-dismiss also, but keep fallback
    setTimeout(() => { toasts = toasts.filter((t) => t.id !== id) }, 5500)
  }

  function handleCreated(e: CustomEvent) {
    const { name, db, warn } = e.detail as { name: string; db?: string; warn?: string }
    const port = apachePort ?? 8080
    if (warn) {
      addToast(warn, 'error')
    } else {
      addToast(MSG.created(name, port, db))
    }
    refreshProjects(port)
  }

  function handleProjectToast(e: CustomEvent) {
    const { msg, kind } = e.detail as { msg: string; kind?: 'info' | 'error' }
    addToast(msg, kind ?? 'info')
  }

  function handleEmptyCta() {
    const el = document.getElementById('create-input')
    el?.focus()
    el?.scrollIntoView({ behavior: 'smooth', block: 'center' })
  }

  function handleServiceConflict(e: CustomEvent) {
    const { conflicts, error } = e.detail as { conflicts: ConflictInfo[]; error?: string }
    modalConflicts = conflicts
    modalError = error ?? ''
    modalOpen = true
  }

  function handleServiceError(e: CustomEvent) {
    const { msg } = e.detail as { msg: string }
    addToast(msg, 'error')
  }

  function handleServiceToast(e: CustomEvent) {
    const { msg } = e.detail as { msg: string }
    addToast(msg, 'info')
  }

  function handleOpenLogViewer(e: CustomEvent) {
    const { service } = e.detail as { service: 'apache' | 'mysql' | 'php' }
    logService = service ?? 'apache'
    logOpen = true
  }

  async function handleUseSuggest(e: CustomEvent) {
    const c = e.detail as ConflictInfo
    if (c.name === 'apache') {
      services.update((s) => ({ ...s, apachePort: c.suggest }))
    } else {
      services.update((s) => ({ ...s, mysqlPort: c.suggest }))
    }
    modalOpen = false
    addToast(`Coba pakai port ${c.suggest} buat ${c.name.toUpperCase()}...`)

    try {
      await startService(c.name, c.suggest)
      addToast(`${c.name.toUpperCase()} nyala di port ${c.suggest} 🎉`)
      if (c.name === 'apache') {
        await refreshProjects(c.suggest)
      }
      await refreshStatus()
    } catch (err) {
      const msg = typeof err === 'string' ? err : String(err)
      modalConflicts = [c]
      modalError = msg
      modalOpen = true
    }
  }

  function handleModalClose() { modalOpen = false }

  async function handleOpenLog() {
    // Open LogViewer for apache by default when conflict modal asks for logs
    logService = modalConflicts[0]?.name ?? 'apache'
    logOpen = true
    modalOpen = false
  }

  let lastApachePort = 8080
  $: {
    const curPort = $services.apachePort
    if (curPort !== lastApachePort) {
      lastApachePort = curPort
      refreshProjects(curPort)
    }
  }
</script>

<div class="min-h-[100dvh] bg-zinc-950 text-zinc-100 px-4 md:px-8 py-8 selection:bg-volt selection:text-black">
  <header class="max-w-[880px] mx-auto flex justify-between items-center mb-10">
    <BrandWordmark />
    <div class="flex items-center gap-3">
      <span class="font-mono text-[10px] tracking-[0.08em] uppercase text-zinc-500">v0.1.0 • Windows portable</span>
      <button
        on:click={()=>{ logService='apache'; logOpen=true }}
        class="rounded-full bg-white/[0.06] ring-1 ring-white/10 px-3 py-1.5 text-[11px] font-mono text-zinc-400 hover:text-white hover:bg-white/[0.10] transition-all"
      >Logs</button>
    </div>
  </header>

  <main class="max-w-[880px] mx-auto flex flex-col gap-6">
    <ServiceCard on:conflict={handleServiceConflict} on:error={handleServiceError} on:toast={handleServiceToast} on:openLogs={handleOpenLogViewer} />

    {#if isEmpty}
      <EmptyState on:cta={handleEmptyCta} />
    {:else}
      <ProjectCard port={apachePort} on:toast={handleProjectToast} />
    {/if}

    <CreateCard on:created={handleCreated} />
  </main>

  <Toast bind:toasts />

  <PortConflictModal
    open={modalOpen}
    conflicts={modalConflicts}
    errorMsg={modalError}
    on:close={handleModalClose}
    on:useSuggest={handleUseSuggest}
    on:openLog={handleOpenLog}
  />

  <LogViewer open={logOpen} service={logService} on:close={()=>logOpen=false} on:toast={handleProjectToast} />
</div>

<style>
  :global(html) { background: #09090b; }
</style>
