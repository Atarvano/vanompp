<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { get } from 'svelte/store'
  import Sidebar from './lib/components/Sidebar.svelte'
  import ServicesPage from './lib/components/ServicesPage.svelte'
  import ProjectPage from './lib/components/ProjectPage.svelte'
  import PengembangPage from './lib/components/PengembangPage.svelte'
  import Toast from './lib/components/Toast.svelte'
  import PortConflictModal from './lib/components/PortConflictModal.svelte'
  import LogViewer from './lib/components/LogViewer.svelte'
  import { refreshProjects } from './lib/stores/projects'
  import { services, refreshStatus, startService, type ConflictInfo, loadPersistedFromRust, setPersistedPort, DEFAULT_APACHE_PORT } from './lib/stores/services'
  import { activePage } from './lib/stores/nav'

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
  function handleProjectToast(e: CustomEvent) {
    const { msg, kind } = e.detail as { msg: string; kind?: 'info' | 'error' }
    addToast(msg, kind ?? 'info')
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
    const { name, suggest } = e.detail as { name: 'apache' | 'mysql'; suggest: number }
    try {
      await setPersistedPort(name, suggest)
      addToast(`Port ${name} diubah ke ${suggest}`, 'info')
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

<div class="min-h-screen bg-[#fdfdf9] text-zinc-900 selection:bg-[#E9FF70] selection:text-black flex">
  <Sidebar />

  <main class="flex-1 min-w-0">
    <div class="mx-auto max-w-[720px] px-4 py-6 md:py-8 space-y-5 pb-20 md:pb-8">
      {#if $activePage === 'services'}
        <ServicesPage on:conflict={handleServiceConflict} on:error={handleServiceError} on:toast={handleServiceToast} on:openLogs={handleOpenLogViewer} />
      {:else if $activePage === 'projects'}
        <ProjectPage on:toast={handleProjectToast} />
      {:else}
        <PengembangPage />
      {/if}
    </div>
  </main>
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
