<script lang="ts">
  import BezelCard from './BezelCard.svelte'
  export let services = { apache:false, mysql:false }
  // Phase 1 mock toggle
  function toggle(svc:'apache'|'mysql'){
    services = { ...services, [svc]: !services[svc as keyof typeof services] }
  }
</script>
<BezelCard>
  <div class="flex justify-between items-start">
    <div>
      <h2 class="text-[11px] font-mono font-semibold tracking-[0.14em] uppercase text-zinc-500 mb-3">Services</h2>
      <div class="flex gap-2.5">
        <button on:click={()=>toggle('apache')} class="group flex items-center gap-2 rounded-full ring-1 ring-white/10 bg-white/[0.04] px-3 py-1.5 text-[12px] font-mono transition-all active:scale-[0.98] hover:ring-white/20">
          <span class="w-2 h-2 rounded-full {services.apache ? 'bg-volt shadow-[0_0_8px_rgba(233,255,112,0.6)]' : 'bg-zinc-600'} transition-colors"></span>
          <span class="text-zinc-300">Apache</span>
          <span class="ml-0.5 text-[10px] {services.apache ? 'text-volt' : 'text-zinc-600'} font-bold">{services.apache ? 'ON' : 'OFF'}</span>
        </button>
        <button on:click={()=>toggle('mysql')} class="group flex items-center gap-2 rounded-full ring-1 ring-white/10 bg-white/[0.04] px-3 py-1.5 text-[12px] font-mono transition-all active:scale-[0.98] hover:ring-white/20">
          <span class="w-2 h-2 rounded-full {services.mysql ? 'bg-volt shadow-[0_0_8px_rgba(233,255,112,0.6)]' : 'bg-zinc-600'} transition-colors"></span>
          <span class="text-zinc-300">MySQL</span>
          <span class="ml-0.5 text-[10px] {services.mysql ? 'text-volt' : 'text-zinc-600'} font-bold">{services.mysql ? 'ON' : 'OFF'}</span>
        </button>
      </div>
      <div class="mt-2.5 flex gap-2 text-[10px] font-mono text-zinc-600">
        <span># apache:8080</span><span># mysql:3306</span>
      </div>
    </div>
    <button class="rounded-full bg-white text-black px-5 py-2.5 text-sm font-semibold flex items-center gap-2 active:scale-[0.98] transition-transform duration-[600ms] ease-[cubic-bezier(0.32,0.72,0,1)] hover:bg-zinc-100">
      Start All
      <span class="w-7 h-7 bg-black/10 rounded-full flex items-center justify-center text-[12px]">↗</span>
    </button>
  </div>
</BezelCard>
