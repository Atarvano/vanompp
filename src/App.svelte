<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { get } from 'svelte/store'
  import ServiceCard from './lib/components/ServiceCard.svelte'
  import ProjectCard from './lib/components/ProjectCard.svelte'
  import CreateCard from './lib/components/CreateCard.svelte'
  import EmptyState from './lib/components/EmptyState.svelte'
  import Toast from './lib/components/Toast.svelte'
  import PortConflictModal from './lib/components/PortConflictModal.svelte'
  import LogViewer from './lib/components/LogViewer.svelte'
  import { projects, refreshProjects } from './lib/stores/projects'
  import { services, refreshStatus, startService, type ConflictInfo, loadPersistedFromRust, setPersistedPort, DEFAULT_APACHE_PORT, DEFAULT_MYSQL_PORT } from './lib/stores/services'
  import { MSG } from './lib/utils/messages'

  type ToastItem = { id: number; msg: string; kind?: 'info' | 'error' }
  let toasts: ToastItem[] = []
  let nextToastId = 1

  let modalOpen = false
  let modalConflicts: ConflictInfo[] = []
  let modalError = ''

  let logOpen = false
  let logService: 'apache' | 'mysql' | 'php' | null = null

  let pollTimer: ReturnType<typeof setInterval> | null = null
  const POLL_MS = 3000

  $: projectList = $projects
  $: isEmpty = projectList.length === 0
  $: apachePort = $services.apachePort

  onMount(async () => {
    // v1.1: load persisted ports from Rust toml + localStorage before first status
    try { await loadPersistedFromRust() } catch {}
    await Promise.all([refreshStatus(), refreshProjects()])
    pollTimer = setInterval(() => {
      refreshStatus().catch(() => {})
    }, POLL_MS)
  })

  onDestroy(() => {
    if (pollTimer) clearInterval(pollTimer)
  })

  function addToast(msg: string, kind: 'info' | 'error' = 'info') {
    const id = nextToastId++
    toasts = [...toasts, { id, msg, kind } as any]
    setTimeout(() => { toasts = toasts.filter((t) => t.id !== id) }, 5500)
  }

  function handleCreated(e: CustomEvent) {
    const { name, db, warn } = e.detail as { name: string; db?: string; warn?: string }
    const port = apachePort ?? 8080
    if (warn) addToast(warn, 'error')
    else addToast(MSG.created(name, port, db))
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
    const c = e.detail as ConflictInfo & { name: 'apache' | 'mysql'; suggest: number }
    // v1.1 Q4 Y: Tetap pakai {suggest} = Pakai & Ingat forever
    try {
      await setPersistedPort(c.name, c.suggest)
    } catch {}
    if (c.name === 'apache') services.update((s) => ({ ...s, apachePort: c.suggest }))
    else services.update((s) => ({ ...s, mysqlPort: c.suggest }))
    modalOpen = false
    addToast(`Tetap pakai ${c.suggest} — Ingat terus ya`)
    try {
      await startService(c.name, c.suggest)
      addToast(`${c.name.toUpperCase()} nyala di port ${c.suggest} 🎉 — Ingat terus`)
      if (c.name === 'apache') await refreshProjects(c.suggest)
      await refreshStatus()
    } catch (err) {
      const msg = typeof err === 'string' ? err : String(err)
      addToast(msg, 'error')
      modalError = msg
      modalOpen = true
    }
  }

  function handleModalClose() {
    modalOpen = false
    modalError = ''
  }

  function handleModalOpenLog() {
    modalOpen = false
    logService = 'apache'
    logOpen = true
  }
</script>

<div class="min-h-screen bg-[#0a0a0a] text-zinc-100 selection:bg-volt selection:text-black">
  <div class="mx-auto max-w-[980px] px-6 md:px-8 py-10 md:py-14">
    <header class="mb-10 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="w-9 h-9 rounded-full bg-white grid place-items-center text-black font-bold text-[13px]">V</div>
        <div>
          <h1 class="text-[16px] font-semibold tracking-tight leading-none">Vanompp</h1>
          <p class="text-[11px] font-mono text-zinc-500 mt-1">Apache + MySQL portable — anti-bingung SMK</p>
        </div>
      </div>
      <div class="text-[10px] font-mono text-zinc-600">v0.1.0</div>
    </header>

    <div class="space-y-6">
      <ServiceCard on:conflict={handleServiceConflict} on:error={handleServiceError} on:toast={handleServiceToast} on:openLogs={handleOpenLogViewer} />

      <div>
        <h2 class="text-[11px] font-mono font-semibold tracking-[0.14em] uppercase text-zinc-500 mb-3">Projects — www/</h2>
        {#if isEmpty}
          <EmptyState on:cta={handleEmptyCta} />
        {:else}
          <ProjectCard on:toast={handleProjectToast} />
        {/if}

        <CreateCard on:created={handleCreated} />
      </div>
    </div>
  </div>

  <Toast bind:toasts />

  <PortConflictModal
    open={modalOpen}
    conflicts={modalConflicts}
    errorMsg={modalError}
    on:close={handleModalClose}
    on:useSuggest={handleUseSuggest}
    on:openLog={handleModalOpenLog}
  />

  <LogViewer bind:open={logOpen} service={logService} on:close={() => (logOpen = false)} />
</div>
