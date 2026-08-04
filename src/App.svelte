<script lang="ts">
  import { onMount } from 'svelte'
  import BrandWordmark from './lib/components/BrandWordmark.svelte'
  import ServiceCard from './lib/components/ServiceCard.svelte'
  import ProjectCard from './lib/components/ProjectCard.svelte'
  import CreateCard from './lib/components/CreateCard.svelte'
  import EmptyState from './lib/components/EmptyState.svelte'
  import Toast from './lib/components/Toast.svelte'

  let services = { apache:false, mysql:false, apachePort:8080, mysqlPort:3306 }
  let projects: {name:string; path?:string; url?:string}[] = []
  let selected = ''
  let toasts: {id:number, msg:string}[] = []
  let nextToastId = 1

  let isEmpty = true

  onMount(()=>{
    // Phase1 mock: check if www empty, show empty state
    isEmpty = projects.length === 0
  })

  function addToast(msg:string){
    const id = nextToastId++
    toasts = [...toasts, { id, msg }]
    setTimeout(()=> { toasts = toasts.filter(t=>t.id!==id) }, 4000)
  }

  function handleCreated(e: CustomEvent){
    const { name, db } = e.detail
    // mock add project
    if (!projects.find(p=>p.name===name)){
      projects = [...projects, { name, path:`www/${name}`, url:`http://localhost:${services.apachePort}/${name}` }]
    }
    selected = name
    isEmpty = false
    addToast(`Project "${name}" dibuat! URL: http://localhost:${services.apachePort}/${name}${db ? ` + DB ${db}` : ''}`)
  }

  function handleEmptyCta(){
    const el = document.getElementById('create-input')
    el?.focus()
    el?.scrollIntoView({ behavior:'smooth', block:'center' })
  }
</script>

<div class="min-h-[100dvh] bg-zinc-950 text-zinc-100 px-4 md:px-8 py-8 selection:bg-volt selection:text-black">
  <header class="max-w-[880px] mx-auto flex justify-between items-center mb-10">
    <BrandWordmark />
    <span class="font-mono text-[10px] tracking-[0.08em] uppercase text-zinc-500">v0.1.0 • Windows portable</span>
  </header>

  <main class="max-w-[880px] mx-auto flex flex-col gap-6">
    <ServiceCard bind:services />

    {#if isEmpty}
      <EmptyState on:cta={handleEmptyCta} />
    {:else}
      <ProjectCard {projects} bind:selected port={services.apachePort} />
    {/if}

    <CreateCard on:created={handleCreated} />
  </main>

  <Toast bind:toasts />
</div>

<style>
  :global(html){ background:#09090B }
</style>
