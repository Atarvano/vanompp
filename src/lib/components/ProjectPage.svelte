<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import ProjectCard from './ProjectCard.svelte'
  import CreateCard from './CreateCard.svelte'
  import EmptyState from './EmptyState.svelte'
  import { projects, refreshProjects } from '$lib/stores/projects'
  import { services } from '$lib/stores/services'
  import { tc, t } from '$lib/utils/messages'
  import { activePage } from '$lib/stores/nav'
  $: tt = $t

  const dispatch = createEventDispatcher<{ toast: { msg: string; kind?: 'info' | 'error' } }>()

  $: projectList = $projects
  $: isEmpty = projectList.length === 0
  $: apachePort = $services.apachePort

  function handleCreated(e: CustomEvent) {
    const { name, db, warn } = e.detail as { name: string; db?: string; warn?: string }
    const port = apachePort ?? 8080
    if (warn) dispatch('toast', { msg: warn, kind: 'error' })
    else dispatch('toast', { msg: tc('created', name, port, db), kind: 'info' })
    refreshProjects(port)
  }
  function handleProjectToast(e: CustomEvent) {
    dispatch('toast', e.detail)
  }
  function handleEmptyCta() {
    activePage.set('projects')
    setTimeout(() => {
      const el = document.getElementById('create-input')
      el?.scrollIntoView({ behavior: 'smooth', block: 'center' })
      setTimeout(() => (el as HTMLInputElement)?.focus(), 250)
    }, 50)
  }
</script>

<div>
  <h2 class="mb-3 font-mono text-[11px] font-semibold uppercase tracking-[0.14em] text-zinc-500">{tt('projectsTitle')}</h2>
  {#if isEmpty}
    <EmptyState on:cta={handleEmptyCta} />
  {:else}
    <ProjectCard on:toast={handleProjectToast} />
  {/if}
  <div class="mt-4">
    <CreateCard on:created={handleCreated} />
  </div>
</div>
