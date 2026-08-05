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
    try { await loadPersistedFromRust() } catch {}
    const cur = get(services)
    const ap = cur.apachePort ?? DEFAULT_APACHE_PORT
    await Promise.all([refreshStatus(), refreshProjects(ap)])
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
    el?.scrollIntoView({ behavior: 'smooth', block: 'center' })
    setTimeout(() => (el as HTMLInputElement)?.focus(), 250)
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
    logService = service as any
    logOpen = true
  }
  async function handleUseSuggest(e: CustomEvent) {
    const { name, suggest, keep } = e.detail as { name: 'apache' | 'mysql'; suggest: number; keep?: boolean }
    try {
      if (keep) {
        await setPersistedPort(name, suggest)
        addToast(`Tetap pakai ${suggest} — ${name} persisted`, 'info')
      } else {
        await setPersistedPort(name, suggest)
        addToast(`Port ${name} diubah ke ${suggest}`, 'info')
      }
      modalOpen = false
      const ap = name === 'apache' ? suggest : (apachePort ?? DEFAULT_APACHE_PORT)
      await Promise.all([refreshStatus(), refreshProjects(ap)])
      await startService(name)
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
    const firstName = modalConflicts[0]?.name
    logService = firstName === 'mysql' ? 'mysql' : 'apache'
    logOpen = true
  }
</script>

<div class="min-h-screen bg-[#fdfdf9] text-zinc-900 selection:bg-[#E9FF70] selection:text-black">
  <div class="mx-auto max-w-[720px] px-4 py-6 md:py-8 space-y-5">
    <header class="flex items-center justify-between">
      <div class="flex items-center gap-2.5">
        <img src="/mascot.png" alt="Vano" class="h-8 w-8 md:h-[32px] md:w-[32px] rounded-[10px] object-cover ring-1 ring-zinc-200 shadow-sm bg-white" />
        <div class="leading-tight">
          <h1 class="text-[14px] font-bold tracking-tight text-zinc-900">vanompp</h1>
          <p class="text-[11px] text-zinc-500">Apache + MySQL portable - anti-bingung SMK</p>
        </div>
      </div>
      <div class="rounded-full border border-zinc-200 bg-white px-2.5 py-0.5 font-mono text-[10px] text-zinc-600">v0.1.0</div>
    </header>

    <ServiceCard on:conflict={handleServiceConflict} on:error={handleServiceError} on:toast={handleServiceToast} on:openLogs={handleOpenLogViewer} />

    <div>
      <h2 class="mb-3 font-mono text-[11px] font-semibold uppercase tracking-[0.14em] text-zinc-500">Projects — www/</h2>
      {#if isEmpty}
        <EmptyState on:cta={handleEmptyCta} />
      {:else}
        <ProjectCard on:toast={handleProjectToast} />
      {/if}
      <div class="mt-4">
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

  <LogViewer bind:open={logOpen} service={logService} on:close={() => (logOpen = false)} on:toast={handleProjectToast} />
</div>
